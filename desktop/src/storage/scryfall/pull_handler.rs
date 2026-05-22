//! Handles streaming scryfall bulk card json into a dataframe that we can use.

// use schemas::oshibana::scryfall::SCRYFALL_SCHEMA;
use crate::storage::scryfall::callback_reader::CallbackReader;
use crate::storage::scryfall::SCRYFALL_DATA_FILE_PATH;
use atomic_float::AtomicF32;
use atomic_time::AtomicInstant;
use schemas::scryfall::card::{ScryfallCard, ScryfallCardBuilder};
use std::fs::File;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use polars::prelude::ParquetWriter;
use struson::reader::{JsonReader, JsonStreamReader};
use url::Url;
use schemas::traits::builder::PolarsBuilder;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum SyncState {
    #[default]
    Idle,
    Downloading,
    FsWrite,
}

#[derive(Debug)]
pub struct PullHandler {
    pub bytes_received: Arc<AtomicUsize>,
    pub cards_transformed: Arc<AtomicUsize>,
    pub sync_state: Arc<Mutex<SyncState>>,
    pub displayed_downloaded: Arc<AtomicUsize>,
    pub displayed_rate: Arc<AtomicF32>,
    last_tick: Arc<AtomicInstant>,
}

impl PullHandler {
    const UPDATE_DISPLAY_INTERVAL: Duration = Duration::from_millis(300);

    pub fn new() -> Self {
        PullHandler {
            bytes_received: Arc::new(Default::default()),
            cards_transformed: Arc::new(Default::default()),
            sync_state: Arc::new(Mutex::new(SyncState::Idle)),
            displayed_downloaded: Arc::new(Default::default()),
            displayed_rate: Arc::new(Default::default()),
            last_tick: Arc::new(AtomicInstant::now()),
        }
    }

    pub async fn pull(
        &self,
        uri: Url
    ) -> anyhow::Result<()> {
        let displayed_download = Arc::clone(&self.displayed_downloaded);
        let displayed_rate = Arc::clone(&self.displayed_rate);
        let cards_transformed = Arc::clone(&self.cards_transformed);
        let last_tick = Arc::clone(&self.last_tick);

        tokio::task::spawn_blocking::<_, anyhow::Result<()>>(move || {
            let start = Instant::now();
            let response = reqwest::blocking::get(uri)?;

            let wrapper_cb = |total_read: usize| {
                if last_tick.load(Ordering::Acquire).elapsed() > Self::UPDATE_DISPLAY_INTERVAL {
                    displayed_download.store(total_read, Ordering::Relaxed);
                    let new_rate = total_read as f32 / start.elapsed().as_secs_f32();
                    displayed_rate.store(new_rate, Ordering::Relaxed);
                    last_tick.store(Instant::now(), Ordering::Release);
                }
            };

            let wrapped_reader = CallbackReader {
                read_bytes: 0,
                cb: wrapper_cb,
                reader: response,
            };

            let mut json_reader = JsonStreamReader::new(wrapped_reader);
            json_reader.begin_array()?;
            let mut builder = <ScryfallCardBuilder as PolarsBuilder<ScryfallCard>>::new();

            while json_reader.has_next()? {
                let card = json_reader.deserialize_next::<ScryfallCard>()?;
                builder.append(card)?;
                cards_transformed.fetch_add(1, Ordering::Relaxed);
            }

            json_reader.end_array()?;
            let mut df = builder.finish_into_dataframe()?;
            let mut file = File::create(&*SCRYFALL_DATA_FILE_PATH)?;
            ParquetWriter::new(&mut file).finish(&mut df)?;
            Ok(())
        }).await?
    }
}
