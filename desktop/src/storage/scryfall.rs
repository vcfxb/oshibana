pub mod callback_reader;
pub mod pull_handler;

use crate::storage::DATA_DIR;
use crate::storage::scryfall::pull_handler::{PullHandler, SyncState};
use anyhow::anyhow;
use clients::scryfall::ScryfallClient;
use polars::prelude::LazyFrame;
use polars::prelude::PlRefPath;
use polars::prelude::PolarsError;
use polars::prelude::PolarsResult;
use polars::prelude::ScanArgsParquet;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::sync::{Arc, LazyLock};

pub static SCRYFALL_DATA_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| DATA_DIR.join("scryfall-data.parquet"));

pub struct ScryfallStorage {
    lf: Option<LazyFrame>,
    client: ScryfallClient,
    pub pull_handler: Arc<PullHandler>,
}

impl ScryfallStorage {
    pub fn new(client: ScryfallClient) -> Self {
        Self {
            lf: Self::load_lf().ok(),
            client: client.clone(),
            pull_handler: Arc::new(PullHandler::new()),
        }
    }

    fn load_lf() -> PolarsResult<LazyFrame> {
        if !SCRYFALL_DATA_FILE_PATH.exists() {
            return Err(PolarsError::ComputeError(
                "scryfall data file does not exist".into(),
            ));
        }

        let pl_path_ref = PlRefPath::try_from_path(&SCRYFALL_DATA_FILE_PATH)?;
        let args = ScanArgsParquet::default();
        LazyFrame::scan_parquet(pl_path_ref, args)
    }

    pub fn trigger_sync(&self) {
        let mut state = self.pull_handler.sync_state.lock().unwrap();

        // Guard against multiple syncs running simultaneously
        if *state != SyncState::Idle {
            return;
        }

        *state = SyncState::Downloading;
        drop(state);

        let sync_state = Arc::clone(&self.pull_handler.sync_state);
        let sync_size = Arc::clone(&self.pull_handler.sync_size);
        let client = self.client.clone();
        let pull_handler = self.pull_handler.clone();

        tokio::task::spawn(async move {
            let res = async {
                let bulk_data = client.bulk_data().await?;
                let all_cards = bulk_data
                    .data
                    .into_iter()
                    .find(|item| item.r#type == "all_cards")
                    .ok_or_else(|| anyhow!("No all_cards bulk data found"))?;

                sync_size.store(all_cards.size as usize, Ordering::Relaxed);
                pull_handler.pull(all_cards.download_uri).await?;

                Ok::<(), anyhow::Error>(())
            }
            .await;

            if let Err(e) = res {
                log::error!("Scryfall sync failed: {}", e);
            }

            let mut state = sync_state.lock().unwrap();
            *state = SyncState::Idle;
        });
    }

    pub fn try_reload(&mut self) -> bool {
        self.lf = Self::load_lf().ok();
        self.lf.is_some()
    }

    pub fn is_ready(&self) -> bool {
        self.lf.is_some()
    }

    pub fn is_syncing(&self) -> bool {
        *self.pull_handler.sync_state.lock().unwrap() != SyncState::Idle
    }
}
