pub mod callback_reader;
pub mod search;
pub mod sync_handler;

use crate::DATA_DIR;
use crate::scryfall::sync_handler::{SyncHandler, SyncState};
use crate::user_data::UserDataStorage;
use anyhow::anyhow;
use chrono::Utc;
use clients::scryfall::ScryfallClient;
use polars::prelude::LazyFrame;
use polars::prelude::PlRefPath;
use polars::prelude::PolarsError;
use polars::prelude::PolarsResult;
use polars::prelude::ScanArgsParquet;
use schemas::scryfall::card::SCRYFALL_CARD_SCHEMA;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock, Mutex};
use std::time::Duration;

pub static SCRYFALL_DATA_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| DATA_DIR.join("scryfall-data.parquet"));

pub struct ScryfallStorage {
    lf: Mutex<Option<LazyFrame>>,
    client: ScryfallClient,
    needs_reload_from_fs: AtomicBool,
    pub sync_handler: Arc<SyncHandler>,
}

// pub struct ScryfallStorage {
//     lf: Option<LazyFrame>,
//     client: ScryfallClient,
//     needs_resync: Arc<AtomicBool>,
//     pub sync_handler: Arc<SyncHandler>,
// }

impl ScryfallStorage {
    pub fn new(client: ScryfallClient) -> Self {
        Self {
            lf: Mutex::new(Self::load_lf().ok()),
            client: client.clone(),
            needs_reload_from_fs: AtomicBool::new(false),
            sync_handler: Arc::new(SyncHandler::new()),
        }
    }

    fn load_lf() -> PolarsResult<LazyFrame> {
        if !SCRYFALL_DATA_FILE_PATH.exists() {
            return Err(PolarsError::ComputeError(
                "scryfall data file does not exist".into(),
            ));
        }

        let pl_path_ref = PlRefPath::try_from_path(&SCRYFALL_DATA_FILE_PATH)?;
        let args = ScanArgsParquet {
            // use_statistics: false,
            ..ScanArgsParquet::default()
        };

        let mut lf = LazyFrame::scan_parquet(pl_path_ref, args)?;
        let actual_schema = lf.collect_schema()?;

        if actual_schema != *SCRYFALL_CARD_SCHEMA {
            return Err(PolarsError::SchemaMismatch(
                "file schema does not match expected".into(),
            ));
        }

        log::info!("Loaded Scryfall lazyframe successfully");
        Ok(lf)
    }

    pub fn trigger_sync(
        self: Arc<Self>,
        userdata: Arc<UserDataStorage>,
        progress_cb: impl Fn(usize, Duration) + Send + 'static,
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
                let bulk_data = client.bulk_data().await?;
                let all_cards = bulk_data
                    .data
                    .into_iter()
                    .find(|item| item.r#type == "all_cards")
                    .ok_or_else(|| anyhow!("No all_cards bulk data found"))?;

                self.sync_handler
                    .expected_size
                    .store(all_cards.size as usize, Ordering::Release);
                let sync_handler = Arc::clone(&self.sync_handler);
                sync_handler
                    .pull(all_cards.download_uri, progress_cb)
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
        let mut lf_guard = self.lf.lock().unwrap();

        *lf_guard = Self::load_lf()
            .map_err(|err| log::warn!("error loading scryfall data, will retry: {err}"))
            .ok();

        let success = lf_guard.is_some();
        drop(lf_guard);
        self.needs_reload_from_fs.store(!success, Ordering::Release);
        success
    }

    pub fn is_ready(&self) -> bool {
        let has_loaded = self.lf.lock().unwrap().is_some();
        has_loaded && !self.needs_reload_from_fs.load(Ordering::Acquire)
    }
}
