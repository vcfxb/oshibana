//! Handles streaming scryfall bulk card json into a dataframe that we can use.

use crate::scryfall::callback_reader::{CallbackReader, ProgressCallback};
use flate2::bufread::GzDecoder;
use humansize::{FormatSizeOptions, format_size};
use polars::prelude::{ParquetWriter, StructChunked};
use schemas::scryfall::bulk_data::BulkDataItem;
use schemas::traits::builder::PolarsBuilder;
use schemas::traits::map_type::MapPolarsType;
use serde::Deserialize;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum SyncState {
    #[default]
    Idle,
    Downloading,
    FsWrite,
}

#[derive(Debug)]
pub struct SyncHandler {
    /// Human-readable string -- what are we syncing from scryfall?
    pub sync_target: Mutex<String>,
    pub sync_state: Mutex<SyncState>,
    pub cancel_requested: AtomicBool,
    pub card_count: AtomicUsize,
    pub expected_size: AtomicUsize,
}

impl SyncHandler {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SyncHandler {
            sync_target: Mutex::new(String::new()),
            sync_state: Mutex::new(SyncState::Idle),
            expected_size: AtomicUsize::new(0),
            card_count: AtomicUsize::new(0),
            cancel_requested: AtomicBool::new(false),
        }
    }

    pub fn is_syncing(&self) -> bool {
        *self.sync_state.lock().unwrap() != SyncState::Idle
    }

    /// Progress Callback args are bytes_synced & duration since start.
    pub async fn pull<D, B>(
        self: Arc<Self>,
        bd_item: &BulkDataItem,
        save_to: &Path,
        progress_cb: impl ProgressCallback + Send + 'static,
    ) -> anyhow::Result<()>
    where
        D: MapPolarsType + for<'de> Deserialize<'de> + Debug,
        B: PolarsBuilder<D, ChunkedType = StructChunked> + Send + 'static,
    {
        self.expected_size
            .store(bd_item.compressed_size as usize, Ordering::Release);
        let bd_name = bd_item.name.clone();
        *self.sync_target.lock().unwrap() = bd_name.clone();
        let mut builder = B::new();
        let uri = bd_item.jsonl_download_uri.clone();
        let save_to = save_to.to_path_buf();

        tokio::task::spawn_blocking::<_, anyhow::Result<()>>(move || {
            log::info!("pulling scryfall {bd_name} data from {uri}");
            let client = reqwest::blocking::Client::builder().build()?;
            let response = client.get(uri).send()?;
            let cb_reader = CallbackReader::new(progress_cb, response);
            // Sticking it in a bufreader gives us a very significant speedup
            let buf_reader = BufReader::new(cb_reader);
            // (2026 Aug 15): Scryfall has switched to gzipped JSONL.
            let gz_reader = GzDecoder::new(buf_reader);
            let lines = BufReader::new(gz_reader).lines();

            for line_result in lines {
                let line = line_result.inspect_err(|err| {
                    log::error!("Could not get next line while reading {bd_name} data: {err}");
                })?;

                let row = serde_json::from_str(line.as_str())?;
                builder.append(row)?;
                self.card_count.fetch_add(1, Ordering::Relaxed);

                if self.cancel_requested.load(Ordering::Relaxed) {
                    log::warn!("Scryfall sync canceled before completion");
                    return Ok(());
                }
            }

            let mut df = builder.finish_into_dataframe()?;
            log::info!("finished downloading {bd_name} data");
            *self.sync_state.lock().unwrap() = SyncState::FsWrite;
            let mut file = File::create(save_to)?;
            ParquetWriter::new(&mut file).finish(&mut df)?;
            log::info!(
                "wrote scryfall {bd_name} data file, {}",
                format_size(file.metadata()?.len(), FormatSizeOptions::default())
            );
            Ok(())
        })
        .await?
    }
}
