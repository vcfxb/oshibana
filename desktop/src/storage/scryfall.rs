//! Scryfall-data is stored

use std::io::Cursor;
use crate::storage::DATA_DIR;
use polars::prelude::{LazyFrame, PlRefPath, PolarsError, PolarsResult, ScanArgsParquet};
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use clients::scryfall::ScryfallClient;
use anyhow::anyhow;
use circular_buffer::CircularBuffer;
use schemas::oshibana::scryfall::SCRYFALL_SCHEMA;

pub static SCRYFALL_DATA_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| DATA_DIR.join("scryfall-data.parquet"));

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum SyncState {
    #[default]
    Idle,
    Downloading,
    FsWrite,
}

pub struct ScryfallStorage {
    lf: Option<LazyFrame>,
    client: ScryfallClient,
    pub sync_size: Arc<AtomicUsize>,
    pub sync_state: Arc<Mutex<SyncState>>,
    pub sync_checkpoints: Arc<Mutex<CircularBuffer<64, (Instant, usize)>>>
}

impl ScryfallStorage {
    pub fn new(client: ScryfallClient) -> Self {
        Self {
            lf: Self::load_lf().ok(),
            client: client.clone(),
            sync_size: Arc::new(AtomicUsize::new(0) ),
            sync_state: Arc::new(Mutex::new(SyncState::Idle)),
            sync_checkpoints: Arc::new(Mutex::new(CircularBuffer::new())),
        }
    }

    fn load_lf() -> PolarsResult<LazyFrame> {
        if !SCRYFALL_DATA_FILE_PATH.exists() {
            return Err(PolarsError::ComputeError("scryfall data file does not exist".into()));
        }

        let pl_path_ref = PlRefPath::try_from_path(&*SCRYFALL_DATA_FILE_PATH)?;
        let args = ScanArgsParquet::default();
        LazyFrame::scan_parquet(pl_path_ref, args)
    }

    pub fn trigger_sync(&self) {
        let mut state = self.sync_state.lock().unwrap();

        // Guard against multiple syncs running simultaneously
        if *state != SyncState::Idle {
            return;
        }

        *state = SyncState::Downloading;
        drop(state);

        let sync_state = Arc::clone(&self.sync_state);
        let sync_size = Arc::clone(&self.sync_size);
        let sync_checkpoints = Arc::clone(&self.sync_checkpoints);
        let client = self.client.clone();

        tokio::task::spawn(async move {
            let res = async {
                let bulk_data = client.bulk_data().await?;
                let all_cards = bulk_data
                    .data
                    .into_iter()
                    .find(|item| item.r#type == "all_cards")
                    .ok_or_else(|| anyhow!("No all_cards bulk data found"))?;

                sync_size.store(all_cards.size as usize, Ordering::SeqCst);

                {
                    let mut checkpoints = sync_checkpoints.lock().unwrap();
                    checkpoints.push_back((Instant::now(), 0));
                }

                let mut response = client.client.get(all_cards.download_uri).send().await?;
                let mut downloaded = 0;

                let mut buffer: Vec<u8> = Vec::with_capacity(all_cards.size as usize);

                while let Some(chunk) = response.chunk().await? {
                    buffer.extend(chunk.as_ref());
                    downloaded += chunk.len();
                    let mut checkpoints = sync_checkpoints.lock().unwrap();
                    checkpoints.push_back((Instant::now(), downloaded));
                }

                {
                    let mut state = sync_state.lock().unwrap();
                    *state = SyncState::FsWrite;
                }

                tokio::task::spawn_blocking(move || {
                    use polars::prelude::*;
                    use std::fs::File;

                    let mut df = JsonReader::new(Cursor::new(&mut buffer))
                        .with_json_format(JsonFormat::Json)
                        .with_schema(Arc::clone(&*SCRYFALL_SCHEMA))
                        .finish()?;
                    
                    let mut file = File::create(&*SCRYFALL_DATA_FILE_PATH)?;
                    ParquetWriter::new(&mut file).finish(&mut df)?;

                    Ok::<(), anyhow::Error>(())
                }).await??;

                Ok::<(), anyhow::Error>(())
            }.await;

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
        *self.sync_state.lock().unwrap() != SyncState::Idle
    }
}
