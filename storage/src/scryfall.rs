pub mod callback_reader;
pub mod search;
pub mod sync_handler;

use crate::DATA_DIR;
use crate::scryfall::callback_reader::ProgressCallback;
use crate::scryfall::sync_handler::{SyncHandler, SyncState};
use crate::user_data::UserDataStorage;
use anyhow::anyhow;
use chrono::Utc;
use clients::scryfall::ScryfallClient;
use polars::prelude::PlRefPath;
use polars::prelude::PolarsError;
use polars::prelude::PolarsResult;
use polars::prelude::ScanArgsParquet;
use polars::prelude::{LazyFrame, SchemaRef};
use schemas::scryfall::card::{SCRYFALL_CARD_SCHEMA, ScryfallCard, ScryfallCardBuilder};
use schemas::scryfall::rulings::{SCRYFALL_RULING_SCHEMA, ScryfallRuling, ScryfallRulingBuilder};
use schemas::scryfall::tags::{SCRYFALL_TAGS_SCHEMA, ScryfallTag, ScryfallTagBuilder};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock, Mutex};

// todo: make sure this path is instantiated
pub static SCRYFALL_DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| DATA_DIR.join("scryfall"));

pub static SCRYFALL_CARD_DATA_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| SCRYFALL_DATA_DIR.join("scryfall_card_data.parquet"));

pub static SCRYFALL_RULINGS_DATA_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| SCRYFALL_DATA_DIR.join("scryfall_rulings_data.parquet"));

pub static SCRYFALL_ART_TAGS_DATA_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| SCRYFALL_DATA_DIR.join("scryfall_art_tag_data.parquet"));

pub static SCRYFALL_ORACLE_TAGS_DATA_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| SCRYFALL_DATA_DIR.join("scryfall_oracle_tags_data.parquet"));

pub struct ScryfallStorage {
    cards_lf: Mutex<Option<LazyFrame>>,
    rulings_lf: Mutex<Option<LazyFrame>>,
    art_tags_lf: Mutex<Option<LazyFrame>>,
    oracle_tags_lf: Mutex<Option<LazyFrame>>,
    client: ScryfallClient,
    needs_reload_from_fs: AtomicBool,
    pub sync_handler: Arc<SyncHandler>,
}

impl ScryfallStorage {
    pub fn new(client: ScryfallClient) -> Self {
        Self {
            cards_lf: Mutex::new(Self::load_cards_lf().ok()),
            rulings_lf: Mutex::new(Self::load_rulings_lf().ok()),
            art_tags_lf: Mutex::new(Self::load_art_tags_lf().ok()),
            oracle_tags_lf: Mutex::new(Self::load_oracle_tags_lf().ok()),
            client: client.clone(),
            needs_reload_from_fs: AtomicBool::new(false),
            sync_handler: Arc::new(SyncHandler::new()),
        }
    }

    fn load_lf(name: &str, path: &Path, expected_schema: SchemaRef) -> PolarsResult<LazyFrame> {
        if !path.exists() {
            return Err(PolarsError::ComputeError(
                format!("scryfall {name} data file does not exist").into(),
            ));
        }

        let pl_path_ref = PlRefPath::try_from_path(path)?;
        let mut lf = LazyFrame::scan_parquet(pl_path_ref, ScanArgsParquet::default())?;
        let actual_schema = lf.collect_schema()?;

        if actual_schema != expected_schema {
            return Err(PolarsError::SchemaMismatch(
                format!("{name} file schema does not match expected").into(),
            ));
        }

        log::info!("Loaded Scryfall {name} lazyframe successfully");
        Ok(lf)
    }

    fn load_rulings_lf() -> PolarsResult<LazyFrame> {
        Self::load_lf(
            "rulings",
            SCRYFALL_RULINGS_DATA_FILE_PATH.as_path(),
            Arc::clone(&*SCRYFALL_RULING_SCHEMA),
        )
    }

    fn load_cards_lf() -> PolarsResult<LazyFrame> {
        Self::load_lf(
            "cards",
            SCRYFALL_CARD_DATA_FILE_PATH.as_path(),
            Arc::clone(&*SCRYFALL_CARD_SCHEMA),
        )
    }

    fn load_art_tags_lf() -> PolarsResult<LazyFrame> {
        Self::load_lf(
            "art tags",
            SCRYFALL_ART_TAGS_DATA_FILE_PATH.as_path(),
            Arc::clone(&*SCRYFALL_TAGS_SCHEMA),
        )
    }

    fn load_oracle_tags_lf() -> PolarsResult<LazyFrame> {
        Self::load_lf(
            "oracle tags",
            SCRYFALL_ORACLE_TAGS_DATA_FILE_PATH.as_path(),
            Arc::clone(&*SCRYFALL_TAGS_SCHEMA),
        )
    }

    fn iter_lazyframes(&self) -> impl Iterator<Item = &Mutex<Option<LazyFrame>>> {
        [
            &self.cards_lf,
            &self.rulings_lf,
            &self.art_tags_lf,
            &self.oracle_tags_lf,
        ]
        .into_iter()
    }

    pub fn trigger_sync(
        self: Arc<Self>,
        userdata: Arc<UserDataStorage>,
        progress_cb: impl ProgressCallback + Clone + Send + 'static,
    ) {
        let mut state = self.sync_handler.sync_state.lock().unwrap();

        // Guard against multiple syncs running simultaneously
        if *state != SyncState::Idle {
            return;
        }

        *state = SyncState::Downloading;
        drop(state);

        log::info!("Scryfall Sync triggered");
        let client = self.client.clone();

        tokio::spawn(async move {
            let res = async {
                let bulk_data = client.bulk_data().await?.data;

                let get_bd_item = |bd_type: &str| {
                    bulk_data
                        .iter()
                        .find(|bd_item| bd_item.r#type == bd_type)
                        .ok_or_else(|| anyhow!("No {bd_type} bulk data found"))
                };

                let arc_clone = Arc::clone(&self.sync_handler);
                let bd_item = get_bd_item("all_cards")?;
                arc_clone
                    .pull::<ScryfallCard, ScryfallCardBuilder>(
                        bd_item,
                        SCRYFALL_CARD_DATA_FILE_PATH.as_path(),
                        progress_cb.clone(),
                    )
                    .await?;

                let arc_clone = Arc::clone(&self.sync_handler);
                let bd_item = get_bd_item("rulings")?;
                arc_clone
                    .pull::<ScryfallRuling, ScryfallRulingBuilder>(
                        bd_item,
                        SCRYFALL_RULINGS_DATA_FILE_PATH.as_path(),
                        progress_cb.clone(),
                    )
                    .await?;

                let arc_clone = Arc::clone(&self.sync_handler);
                let bd_item = get_bd_item("art_tags")?;
                arc_clone
                    .pull::<ScryfallTag, ScryfallTagBuilder>(
                        bd_item,
                        SCRYFALL_ART_TAGS_DATA_FILE_PATH.as_path(),
                        progress_cb.clone(),
                    )
                    .await?;

                let arc_clone = Arc::clone(&self.sync_handler);
                let bd_item = get_bd_item("oracle_tags")?;
                arc_clone
                    .pull::<ScryfallTag, ScryfallTagBuilder>(
                        bd_item,
                        SCRYFALL_ORACLE_TAGS_DATA_FILE_PATH.as_path(),
                        progress_cb,
                    )
                    .await?;

                userdata
                    .loaded
                    .lock()
                    .unwrap()
                    .last_scryfall_sync
                    .replace(Utc::now());

                userdata.trigger_save();
                Ok::<(), anyhow::Error>(())
            }
            .await;

            if let Err(e) = res {
                log::error!("Scryfall Sync failed: {}", e);
            }

            self.needs_reload_from_fs.store(true, Ordering::Release);
            *self.sync_handler.sync_state.lock().unwrap() = SyncState::Idle;
        });
    }

    pub fn try_reload(&self) -> bool {
        let handle_err = |name: &str, f: fn() -> PolarsResult<LazyFrame>| {
            f().inspect_err(|err| log::warn!("error loading scryfall {name} data: {err}"))
                .ok()
        };

        let store_to_mutex = |mu: &Mutex<Option<LazyFrame>>, v: Option<LazyFrame>| {
            let mut guard = mu.lock().unwrap();
            *guard = v;
            guard.is_some()
        };

        let loaded_cards = store_to_mutex(&self.cards_lf, handle_err("cards", Self::load_cards_lf));
        let loaded_rulings = store_to_mutex(
            &self.rulings_lf,
            handle_err("rulings", Self::load_rulings_lf),
        );
        let loaded_atags = store_to_mutex(
            &self.art_tags_lf,
            handle_err("art tags", Self::load_art_tags_lf),
        );
        let loaded_otags = store_to_mutex(
            &self.oracle_tags_lf,
            handle_err("oracle tags", Self::load_oracle_tags_lf),
        );

        let success = loaded_rulings && loaded_cards && loaded_atags && loaded_otags;
        self.needs_reload_from_fs.store(!success, Ordering::Release);
        success
    }

    pub fn is_ready(&self) -> bool {
        let check_loaded = |mu: &Mutex<Option<LazyFrame>>| mu.lock().unwrap().is_some();
        let all_loaded = self.iter_lazyframes().all(check_loaded);
        all_loaded && !self.needs_reload_from_fs.load(Ordering::Acquire)
    }
}
