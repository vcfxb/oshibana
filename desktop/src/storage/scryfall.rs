//! Scryfall-data is stored

use std::fs::File;
use crate::storage::DATA_DIR;
use polars::prelude::{col, DataFrame, LazyFrame, ParquetWriter, PlRefPath, PolarsError, PolarsResult, ScanArgsParquet};
use schemas::oshibana::scryfall::SCRYFALL_SCHEMA;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex};
use std::sync::atomic::AtomicUsize;
use std::sync::mpsc::Receiver;
use std::time::Instant;
use clients::scryfall::ScryfallClient;

static SCRYFALL_DATA_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| DATA_DIR.join("scryfall-data.parquet"));

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
enum SyncState {
    #[default]
    Idle,
    Downloading,
    FsWrite,
}

pub struct ScryfallStorage {
    lf: LazyFrame,
    client: ScryfallClient,
    download_started: Option<Instant>,
    download_size: Arc<(AtomicUsize, AtomicUsize)>,
    sync_state: Arc<Mutex<SyncState>>,
}

impl ScryfallStorage {
    pub fn trigger_sync(&mut self) {
        // block all new syncs on not having one in progress already
        let sync_state_lock = self.sync_state.lock().unwrap();
        if *sync_state_lock != SyncState::Idle {
            return;
        }

        drop(sync_state_lock);
        let sync_state_arc = Arc::clone(&self.sync_state);
        let download_size_arc = Arc::clone(&self.download_size);


        tokio::task::spawn(async move {
        })


    }

    /// Attempt to open the scryfall data file in the app's data path.
    pub fn open(client: ScryfallClient) -> PolarsResult<Self> {
        if !SCRYFALL_DATA_FILE_PATH.exists() {
            log::info!("scryfall data file didn't exist, creating new empty");
            let mut df = DataFrame::empty_with_schema(&*SCRYFALL_SCHEMA);
            let mut file = File::create(&*SCRYFALL_DATA_FILE_PATH)?;
            let writer = ParquetWriter::new(&mut file);
            writer.finish(&mut df)?;
        }

        let pl_path_ref = PlRefPath::try_from_path(&*SCRYFALL_DATA_FILE_PATH)?;
        let args = ScanArgsParquet::default();
        let mut lf = LazyFrame::scan_parquet(pl_path_ref, args)?;
        let collected_schema = lf.collect_schema()?;

        if collected_schema != *SCRYFALL_SCHEMA {
            return Err(PolarsError::SchemaMismatch("loaded lf does not match expected".into()));
        }

        Ok(Self {
            lf,
            client,
            sync_state: Arc::new(Mutex::new(SyncState::Idle)),
            download_size: Default::default(),
            download_started: None
        })
    }
}
