//! Handles streaming scryfall bulk card json into a dataframe that we can use.

use crate::storage::scryfall::callback_reader::CallbackReader;
// use schemas::oshibana::scryfall::SCRYFALL_SCHEMA;
use crate::storage::scryfall::SCRYFALL_DATA_FILE_PATH;
use atomic_float::AtomicF32;
use atomic_time::AtomicInstant;
use polars::prelude::ParquetWriter;
use schemas::scryfall::card::{ScryfallCard, ScryfallCardBuilder};
use schemas::traits::builder::PolarsBuilder;
use serde_json::Deserializer;
use std::fs::File;
use std::io::BufReader;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use humansize::{format_size, FormatSizeOptions};
use url::Url;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum SyncState {
    #[default]
    Idle,
    Downloading,
    Transforming,
    FsWrite,
}

#[derive(Debug)]
pub struct PullHandler {
    pub sync_state: Arc<Mutex<SyncState>>,
    pub total_cards: Arc<AtomicUsize>,
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
            total_cards: Arc::new(Default::default()),
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
        let cards_transformed = Arc::clone(&self.displayed_cards_transformed);
        let last_tick = Arc::clone(&self.last_tick);
        let sync_state = Arc::clone(&self.sync_state);
        let total_cards = Arc::clone(&self.total_cards);
        let sync_size = Arc::clone(&self.sync_size);

        tokio::task::spawn_blocking::<_, anyhow::Result<()>>(move || {
            log::info!("pulling scryfall card data from {uri}");
            let response = reqwest::blocking::get(uri)?;

            let wrapper_cb = |total_read: usize, elapsed: Duration| {
                if last_tick.load(Ordering::Acquire).elapsed() > Self::UPDATE_DISPLAY_INTERVAL {
                    displayed_download.store(total_read, Ordering::Relaxed);
                    let new_rate = total_read as f32 / elapsed.as_secs_f32();
                    displayed_rate.store(new_rate, Ordering::Relaxed);
                    last_tick.store(Instant::now(), Ordering::Release);
                }
            };

            let cb_reader = CallbackReader::new(wrapper_cb, response);
            // Sticking it in a bufreader gives us a very significant speedup
            let buf_reader = BufReader::new(cb_reader);
            let mut deserializer = Deserializer::from_reader(buf_reader);
            let cards: Vec<ScryfallCard> = serde::Deserialize::deserialize(&mut deserializer)?;
            drop(deserializer);
            let num_cards = cards.len();

            log::info!(
                "finished downloading and deserializing, {} downloaded",
                format_size(sync_size.load(Ordering::Relaxed), FormatSizeOptions::default())
            );

            *sync_state.lock().unwrap() = SyncState::Transforming;
            total_cards.store(cards.len(), Ordering::Relaxed);
            let mut builder = <ScryfallCardBuilder as PolarsBuilder<ScryfallCard>>::new();

            for (index, card) in cards.into_iter().enumerate() {
                builder.append(card)?;

                if last_tick.load(Ordering::Acquire).elapsed() > Self::UPDATE_DISPLAY_INTERVAL {
                    cards_transformed.store(index + 1, Ordering::Relaxed);
                    last_tick.store(Instant::now(), Ordering::Release);
                }
            }

            cards_transformed.store(num_cards, Ordering::Relaxed);
            let mut df = builder.finish_into_dataframe()?;
            log::info!("finished transforming cards into in memory dataframe");
            *sync_state.lock().unwrap() = SyncState::FsWrite;
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
