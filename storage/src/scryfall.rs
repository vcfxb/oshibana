pub mod callback_reader;
pub mod search;
pub mod sync_handler;

use crate::scryfall::callback_reader::ProgressCallback;
use crate::scryfall::sync_handler::{SyncHandler, SyncState};
use crate::user_data::UserDataStorage;
use crate::{CACHE_DIR, DATA_DIR};
use anyhow::anyhow;
use chrono::Utc;
use clients::scryfall::ScryfallClient;
use polars::prelude::PolarsError;
use polars::prelude::PolarsResult;
use polars::prelude::SchemaRef;
use polars::prelude::SerReader;
use polars::prelude::{DataFrame, ParquetReader};
use reqwest::Method;
use schemas::scryfall::card::{SCRYFALL_CARD_SCHEMA, ScryfallCard, ScryfallCardBuilder};
use schemas::scryfall::rulings::{SCRYFALL_RULING_SCHEMA, ScryfallRuling, ScryfallRulingBuilder};
use schemas::scryfall::symbology::CardSymbol;
use schemas::scryfall::tags::{SCRYFALL_TAGS_SCHEMA, ScryfallTag, ScryfallTagBuilder};
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock, Mutex};
use std::time::Instant;
use url::Url;

// todo: make sure this path is instantiated
pub static SCRYFALL_DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| DATA_DIR.join("scryfall"));

pub static SCRYFALL_CARD_DATA_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| SCRYFALL_DATA_DIR.join("scryfall_card_data.parquet"));

pub static SCRYFALL_RULINGS_DATA_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| SCRYFALL_DATA_DIR.join("scryfall_rulings_data.parquet"));

pub static SCRYFALL_ART_TAGS_DATA_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| SCRYFALL_DATA_DIR.join("scryfall_art_tag_data.parquet"));

pub static SCRYFALL_ORACLE_TAGS_DATA_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| SCRYFALL_DATA_DIR.join("scryfall_oracle_tag_data.parquet"));

pub static SCRYFALL_SYMBOLOGY_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| SCRYFALL_DATA_DIR.join("scryfall_symbology.json"));

pub static SCRYFALL_SYMBOLOGY_CACHE_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| CACHE_DIR.join("symbology-cache"));

pub static SCRYFALL_CARD_IMAGERY_CACHE_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| CACHE_DIR.join("card-imagery-cache"));

pub struct ScryfallStorage {
    cards_df: Mutex<Option<DataFrame>>,
    rulings_df: Mutex<Option<DataFrame>>,
    art_tags_df: Mutex<Option<DataFrame>>,
    oracle_tags_df: Mutex<Option<DataFrame>>,
    symbology: Mutex<Option<Vec<CardSymbol>>>,
    client: ScryfallClient,
    needs_reload_from_fs: AtomicBool,
    pub sync_handler: Arc<SyncHandler>,
}

impl ScryfallStorage {
    pub fn new(client: ScryfallClient) -> Self {
        Self {
            cards_df: Mutex::new(Self::load_cards_df().ok()),
            rulings_df: Mutex::new(Self::load_rulings_df().ok()),
            art_tags_df: Mutex::new(Self::load_art_tags_df().ok()),
            oracle_tags_df: Mutex::new(Self::load_oracle_tags_df().ok()),
            symbology: Mutex::new(Self::load_symbology().ok()),
            client: client.clone(),
            needs_reload_from_fs: AtomicBool::new(false),
            sync_handler: Arc::new(SyncHandler::new()),
        }
    }

    fn load_df(name: &str, path: &Path, expected_schema: SchemaRef) -> PolarsResult<DataFrame> {
        if !path.exists() {
            return Err(PolarsError::ComputeError(
                format!("scryfall {name} data file does not exist").into(),
            ));
        }

        let file_reader = BufReader::new(File::open(path)?);
        let df = ParquetReader::new(file_reader).set_rechunk(true).finish()?;

        let actual_schema = df.schema();

        if *actual_schema != expected_schema {
            return Err(PolarsError::SchemaMismatch(
                format!("{name} file schema does not match expected").into(),
            ));
        }

        log::info!("Loaded Scryfall {name} lazyframe successfully");
        Ok(df)
    }

    fn load_rulings_df() -> PolarsResult<DataFrame> {
        Self::load_df(
            "rulings",
            SCRYFALL_RULINGS_DATA_FILE_PATH.as_path(),
            Arc::clone(&*SCRYFALL_RULING_SCHEMA),
        )
    }

    fn load_cards_df() -> PolarsResult<DataFrame> {
        Self::load_df(
            "cards",
            SCRYFALL_CARD_DATA_FILE_PATH.as_path(),
            Arc::clone(&*SCRYFALL_CARD_SCHEMA),
        )
    }

    fn load_art_tags_df() -> PolarsResult<DataFrame> {
        Self::load_df(
            "art tags",
            SCRYFALL_ART_TAGS_DATA_FILE_PATH.as_path(),
            Arc::clone(&*SCRYFALL_TAGS_SCHEMA),
        )
    }

    fn load_oracle_tags_df() -> PolarsResult<DataFrame> {
        Self::load_df(
            "oracle tags",
            SCRYFALL_ORACLE_TAGS_DATA_FILE_PATH.as_path(),
            Arc::clone(&*SCRYFALL_TAGS_SCHEMA),
        )
    }

    fn load_symbology() -> anyhow::Result<Vec<CardSymbol>> {
        if !SCRYFALL_SYMBOLOGY_FILE_PATH.exists() {
            anyhow::bail!("symbology file does not exist");
        }

        let start = Instant::now();
        log::info!(
            "reading symbology data from {}",
            SCRYFALL_SYMBOLOGY_FILE_PATH.display()
        );

        let file_content = fs::read(&*SCRYFALL_SYMBOLOGY_FILE_PATH).map_err(|err| {
            log::warn!("failed to read file: {err}");
            err
        })?;

        let data = serde_json::from_slice(file_content.as_slice()).map_err(|err| {
            log::warn!("failed to deserialize: {err}");
            err
        })?;

        log::info!("read symbology data successfully in {:?}", start.elapsed());
        Ok(data)
    }

    fn iter_dataframes(&self) -> impl Iterator<Item = &Mutex<Option<DataFrame>>> {
        [
            &self.cards_df,
            &self.rulings_df,
            &self.art_tags_df,
            &self.oracle_tags_df,
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

                *self.sync_handler.sync_target.lock().unwrap() = "Symbology".to_owned();
                *self.sync_handler.sync_state.lock().unwrap() = SyncState::Downloading;
                let symbology = self.client.symbology().await?;
                *self.sync_handler.sync_state.lock().unwrap() = SyncState::FsWrite;
                let mut file = File::create(SCRYFALL_SYMBOLOGY_FILE_PATH.as_path())?;
                serde_json::to_writer(&mut file, &symbology.data)?;
                log::info!("wrote symbology file");

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
        fn handle_err<T, E: Display, F: FnOnce() -> Result<T, E>>(name: &str, f: F) -> Option<T> {
            f().inspect_err(|err| log::warn!("error loading scryfall {name} data: {err}"))
                .ok()
        }

        fn store_to_mutex<T>(mu: &Mutex<Option<T>>, v: Option<T>) -> bool {
            let mut guard = mu.lock().unwrap();
            *guard = v;
            guard.is_some()
        }

        let loaded_cards = store_to_mutex(&self.cards_df, handle_err("cards", Self::load_cards_df));
        let loaded_rulings = store_to_mutex(
            &self.rulings_df,
            handle_err("rulings", Self::load_rulings_df),
        );
        let loaded_atags = store_to_mutex(
            &self.art_tags_df,
            handle_err("art tags", Self::load_art_tags_df),
        );
        let loaded_otags = store_to_mutex(
            &self.oracle_tags_df,
            handle_err("oracle tags", Self::load_oracle_tags_df),
        );
        let loaded_symbology = store_to_mutex(
            &self.symbology,
            handle_err("symbology", Self::load_symbology),
        );

        let success =
            loaded_rulings && loaded_cards && loaded_atags && loaded_otags && loaded_symbology;
        self.needs_reload_from_fs.store(!success, Ordering::Release);
        success
    }

    pub fn is_ready(&self) -> bool {
        let check_loaded = |mu: &Mutex<Option<DataFrame>>| mu.lock().unwrap().is_some();
        let all_dataframes_loaded = self.iter_dataframes().all(check_loaded);
        let symbology_loaded = self.symbology.lock().unwrap().is_some();
        all_dataframes_loaded
            && symbology_loaded
            && !self.needs_reload_from_fs.load(Ordering::Acquire)
    }

    /// Lookup the svg url of a symbol. Returns `None` if symbology isn't loaded or if the symbol
    /// doesn't have an svg.
    fn lookup_symbol_svg_url(&self, symbol: &str) -> Option<Url> {
        let symbology_guard = self.symbology.lock().unwrap();
        let symbols = symbology_guard.as_ref()?;
        symbols
            .iter()
            .find(|symbol_object| symbol_object.symbol == symbol)
            .and_then(|symbol_object| symbol_object.svg_uri.as_ref())
            .cloned()
    }

    /// Get a [`Url`] for a symbol's svg if it exists. It may be cached, in which case, it's a
    /// `file://` uri.
    pub fn get_symbol_svg_uri(&self, symbol: &str) -> Option<Url> {
        let symbol_uri = self.lookup_symbol_svg_url(symbol)?;
        let filename = symbol_uri
            .path_segments()
            .expect("scryfall symbol svg uri has path segments")
            .next_back()
            .expect("scryfall symbol svg uri has at least one path segment");

        assert!(filename.ends_with(".svg"), "`{filename}` should be an svg");

        let file_location = SCRYFALL_SYMBOLOGY_CACHE_DIR.join(filename);
        assert!(file_location.is_absolute());

        if file_location.exists() {
            Some(Url::from_file_path(&file_location).unwrap())
        } else {
            let client = self.client.clone();
            let uri = symbol_uri.clone();

            let update_cache = async move || {
                let bytes = client
                    .client
                    .request(Method::GET, uri)
                    .send()
                    .await?
                    .bytes()
                    .await?;

                fs::write(file_location, bytes)?;
                Ok::<_, anyhow::Error>(())
            };

            tokio::spawn(async {
                if let Err(err) = update_cache().await {
                    log::warn!("Error updating symbology cache: {err}");
                }
            });

            Some(symbol_uri)
        }
    }
}
