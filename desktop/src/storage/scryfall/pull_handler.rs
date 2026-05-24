//! Handles streaming scryfall bulk card json into a dataframe that we can use.

use crate::storage::scryfall::callback_reader::CallbackReader;
// use schemas::oshibana::scryfall::SCRYFALL_SCHEMA;
use crate::storage::scryfall::SCRYFALL_DATA_FILE_PATH;
use atomic_float::AtomicF32;
use atomic_time::AtomicInstant;
use polars::prelude::ParquetWriter;
use schemas::scryfall::card::{ScryfallCard, ScryfallCardBuilder};
use schemas::traits::builder::PolarsBuilder;
use std::fs::File;
use std::io::BufReader;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use humansize::{format_size, FormatSizeOptions};
use struson::reader::{JsonStreamReader, JsonReader};
use url::Url;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum SyncState {
    #[default]
    Idle,
    Downloading,
    FsWrite,
}

#[derive(Debug)]
pub struct PullHandler {
    pub sync_state: Arc<Mutex<SyncState>>,
    pub displayed_rate: Arc<AtomicF32>,
    pub displayed_bytes: Arc<AtomicUsize>,
    pub displayed_cards_transformed: Arc<AtomicUsize>,
    pub sync_size: Arc<AtomicUsize>,
    last_tick: Arc<AtomicInstant>,
}

impl PullHandler {
    const UPDATE_DISPLAY_INTERVAL: Duration = Duration::from_millis(300);

    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        PullHandler {
            displayed_cards_transformed: Arc::new(Default::default()),
            sync_state: Arc::new(Mutex::new(SyncState::Idle)),
            displayed_bytes: Arc::new(Default::default()),
            displayed_rate: Arc::new(Default::default()),
            sync_size: Arc::new(Default::default()),
            last_tick: Arc::new(AtomicInstant::now()),
        }
    }

    pub async fn pull(&self, uri: Url) -> anyhow::Result<()> {
        let displayed_download = Arc::clone(&self.displayed_bytes);
        let displayed_rate = Arc::clone(&self.displayed_rate);
        let displayed_cards_transformed = Arc::clone(&self.displayed_cards_transformed);
        let last_tick = Arc::clone(&self.last_tick);
        let sync_state = Arc::clone(&self.sync_state);
        let sync_size = self.sync_size.load(Ordering::Relaxed);

        tokio::task::spawn_blocking::<_, anyhow::Result<()>>(move || {
            log::info!("pulling scryfall card data from {uri}");
            let response = reqwest::blocking::get(uri)?;
            let atomic_card_count = AtomicUsize::new(0);
            let mut builder = <ScryfallCardBuilder as PolarsBuilder<ScryfallCard>>::new();

            let wrapper_cb = |total_read: usize, elapsed: Duration| {
                if last_tick.load(Ordering::Acquire).elapsed() > Self::UPDATE_DISPLAY_INTERVAL {
                    displayed_download.store(total_read, Ordering::Relaxed);
                    let new_rate = total_read as f32 / elapsed.as_secs_f32();
                    displayed_rate.store(new_rate, Ordering::Relaxed);
                    let card_count = atomic_card_count.load(Ordering::Acquire);
                    displayed_cards_transformed.store(card_count, Ordering::Relaxed);
                    last_tick.store(Instant::now(), Ordering::Release);
                }
            };

            let cb_reader = CallbackReader::new(wrapper_cb, response);
            // Sticking it in a bufreader gives us a very significant speedup
            let buf_reader = BufReader::new(cb_reader);
            // use struson to iterate over values so that we don't have to build
            // an entire card vec in memory
            let mut json_reader = JsonStreamReader::new(buf_reader);
            json_reader.begin_array()?;

            while json_reader.has_next()? {
                let card = json_reader.deserialize_next::<ScryfallCard>()?;
                builder.append(card)?;
                atomic_card_count.fetch_add(1, Ordering::Release);
            }

            let card_count = atomic_card_count.load(Ordering::Acquire);
            displayed_cards_transformed.store(card_count, Ordering::Relaxed);
            json_reader.end_array()?;
            displayed_download.store(sync_size, Ordering::Relaxed);
            let mut df = builder.finish_into_dataframe()?;
            log::info!("finished downloading card data");
            *sync_state.lock().unwrap() = SyncState::FsWrite;
            let mut file = File::create(&*SCRYFALL_DATA_FILE_PATH)?;
            ParquetWriter::new(&mut file).finish(&mut df)?;
            log::info!(
                "wrote scryfall data file, {}",
                format_size(file.metadata()?.len(), FormatSizeOptions::default())
            );
            Ok(())
        }).await?
    }
}
