//! Handles streaming scryfall bulk card json into a dataframe that we can use.

use crate::scryfall::SCRYFALL_DATA_FILE_PATH;
use crate::scryfall::callback_reader::CallbackReader;
use humansize::{FormatSizeOptions, format_size};
use polars::prelude::ParquetWriter;
use schemas::scryfall::card::{ScryfallCard, ScryfallCardBuilder};
use schemas::traits::builder::PolarsBuilder;
use std::fs::File;
use std::io::BufReader;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use struson::reader::{JsonReader, JsonStreamReader};
use url::Url;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum SyncState {
    #[default]
    Idle,
    Downloading,
    FsWrite,
}

#[derive(Debug)]
pub struct SyncHandler {
    pub sync_state: Mutex<SyncState>,
    pub cancel_requested: AtomicBool,
    pub card_count: AtomicUsize,
    pub expected_size: AtomicUsize,
}

impl SyncHandler {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SyncHandler {
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
    pub async fn pull(
        self: Arc<Self>,
        uri: Url,
        progress_cb: impl Fn(usize, Duration) + Send + 'static,
    ) -> anyhow::Result<()> {
        let mut builder = <ScryfallCardBuilder as PolarsBuilder<ScryfallCard>>::new();

        tokio::task::spawn_blocking::<_, anyhow::Result<()>>(move || {
            log::info!("pulling scryfall card data from {uri}");
            let client = reqwest::blocking::Client::builder().build()?;
            let response = client.get(uri).send()?;
            let cb_reader = CallbackReader::new(progress_cb, response);
            // Sticking it in a bufreader gives us a very significant speedup
            let buf_reader = BufReader::new(cb_reader);
            // use struson to iterate over values so that we don't have to build
            // an entire card vec in memory
            let mut json_reader = JsonStreamReader::new(buf_reader);
            json_reader.begin_array()?;

            while json_reader.has_next()? {
                let card = json_reader.deserialize_next::<ScryfallCard>()?;
                builder.append(card)?;
                self.card_count.fetch_add(1, Ordering::Release);

                if self.cancel_requested.load(Ordering::Relaxed) {
                    log::warn!("Scryfall sync canceled before completion");
                    return Ok(());
                }
            }

            json_reader.end_array()?;
            let mut df = builder.finish_into_dataframe()?;
            log::info!("finished downloading card data");
            *self.sync_state.lock().unwrap() = SyncState::FsWrite;
            let mut file = File::create(&*SCRYFALL_DATA_FILE_PATH)?;
            ParquetWriter::new(&mut file).finish(&mut df)?;
            log::info!(
                "wrote scryfall data file, {}",
                format_size(file.metadata()?.len(), FormatSizeOptions::default())
            );
            Ok(())
        })
        .await?
    }
}
