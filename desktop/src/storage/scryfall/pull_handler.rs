//! Handles streaming scryfall bulk card json into a dataframe that we can use.

use std::borrow::Cow;
use std::fs::File;
use std::io;
use std::io::Read;
use atomic_float::AtomicF32;
use atomic_time::AtomicInstant;
use polars::prelude::{AnyValue, Categorical32Type, Categorical8Type, ChunkCast, ChunkedArray, DataFrame, ListChunked, ListPrimitiveChunkedBuilder, NamedFrom, ParquetWriter, PolarsResult, PrimitiveChunkedBuilder, SchemaExt, StringChunkedBuilder, UInt128Type, UInt64Type};
use schemas::scryfall::card::{colors, finishes, frame, games, image_status, layout, legalities, rarity, security_stamp, ScryfallCard};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use polars::chunked_array::builder::CategoricalChunkedBuilder;
use polars::series::Series;
use struson::reader::{JsonReader, JsonStreamReader};
use url::Url;
use schemas::oshibana::scryfall::SCRYFALL_SCHEMA;
use crate::storage::scryfall::SCRYFALL_DATA_FILE_PATH;

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

struct ReadWrapper<F: Fn(usize), R: Read> {
    read_bytes: usize,
    cb: F,
    reader: R,
}

impl<F: Fn(usize), R: Read> Read for ReadWrapper<F, R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let count = self.reader.read(buf)?;
        self.read_bytes += count;
        (self.cb)(self.read_bytes);
        Ok(count)
    }
}

struct ScryfallDfBuilder {
    arena_id_vec: PrimitiveChunkedBuilder<UInt64Type>,
    id_vec: PrimitiveChunkedBuilder<UInt128Type>,
    lang_vec: CategoricalChunkedBuilder<Categorical8Type>,
    mtgo_id_vec: PrimitiveChunkedBuilder<UInt64Type>,
    mtgo_foil_id_vec: PrimitiveChunkedBuilder<UInt64Type>,
    multiverse_ids_vec: ListPrimitiveChunkedBuilder<UInt64Type>,
    resource_id_vec: StringChunkedBuilder,
    tcgplayer_id_vec: PrimitiveChunkedBuilder<UInt64Type>,
    tcgplayer_etched_id_vec: PrimitiveChunkedBuilder<UInt64Type>,
    cardmarket_id_vec: PrimitiveChunkedBuilder<UInt64Type>,
    layout_vec: CategoricalChunkedBuilder<Categorical8Type>,
    oracle_id_vec: PrimitiveChunkedBuilder<UInt128Type>,
    prints_search_uri_vec: StringChunkedBuilder,
    // rulings_uri,
    // scryfall_uri,
    // uri,
    // all_parts,
    // card_faces,
    // cmc,
    // color_identity,
    // color_indicator,
    // colors,
    // defense,
    // edhrec_rank,
    // game_changer,
    // hand_modifier,
    // keywords,
    // legalities,
    // life_modifier,
    // loyalty,
    // mana_cost,
    // name,
    // oracle_text,
    // penny_rank,
    // power,
    // produced_mana,
    // reserved,
    // toughness,
    // type_line,
    // artist,
    // artist_ids,
    // attraction_lights,
    // booster,
    // border_color,
    // card_back_id,
    // collector_number,
    // content_warning,
    // digital,
    // finishes,
    // flavor_name,
    // flavor_text,
    // frame_effects,
    // frame,
    // full_art,
    // games,
    // highres_image,
    // illustration_id,
    // image_status,
    // image_uris,
    // oversized,
    // prices,
    // printed_name,
    // printed_text,
    // printed_type_line,
    // promo,
    // promo_types,
    // purchase_uris,
    // rarity,
    // related_uris,
    // released_at,
    // reprint,
    // scryfall_set_uri,
    // set_name,
    // set_search_uri,
    // set_type,
    // set_uri,
    // set,
    // set_id,
    // story_spotlight,
    // textless,
    // variation,
    // variation_of,
    // security_stamp,
    // watermark
}

impl ScryfallDfBuilder {
    fn new() -> Self {
        ScryfallDfBuilder {
            arena_id_vec: PrimitiveChunkedBuilder::new(
                SCRYFALL_SCHEMA.get_field("arena_id").unwrap().name,
                0
            ),
            id_vec: (),
            lang_vec: (),
            mtgo_id_vec: (),
            mtgo_foil_id_vec: (),
            multiverse_ids_vec: (),
            resource_id_vec: (),
            tcgplayer_id_vec: (),
            tcgplayer_etched_id_vec: (),
            cardmarket_id_vec: (),
            layout_vec: (),
            oracle_id_vec: (),
            prints_search_uri_vec: (),
        }
    }

    fn push(&mut self, card: ScryfallCard) {
        let ScryfallCard {
            arena_id,
            id,
            lang: lang,
            mtgo_id,
            mtgo_foil_id,
            multiverse_ids,
            resource_id,
            tcgplayer_id,
            tcgplayer_etched_id,
            cardmarket_id,
            layout,
            oracle_id,
            prints_search_uri,
            rulings_uri,
            scryfall_uri,
            uri,
            all_parts,
            card_faces,
            cmc,
            color_identity,
            color_indicator,
            colors,
            defense,
            edhrec_rank,
            game_changer,
            hand_modifier,
            keywords,
            legalities,
            life_modifier,
            loyalty,
            mana_cost,
            name,
            oracle_text,
            penny_rank,
            power,
            produced_mana,
            reserved,
            toughness,
            type_line,
            artist,
            artist_ids,
            attraction_lights,
            booster,
            border_color,
            card_back_id,
            collector_number,
            content_warning,
            digital,
            finishes,
            flavor_name,
            flavor_text,
            frame_effects,
            frame,
            full_art,
            games,
            highres_image,
            illustration_id,
            image_status,
            image_uris,
            oversized,
            prices,
            printed_name,
            printed_text,
            printed_type_line,
            promo,
            promo_types,
            purchase_uris,
            rarity,
            related_uris,
            released_at,
            reprint,
            scryfall_set_uri,
            set_name,
            set_search_uri,
            set_type,
            set_uri,
            set,
            set_id,
            story_spotlight,
            textless,
            variation,
            variation_of,
            security_stamp,
            watermark
        } = card;

        self.id_vec.push(id.as_u128());
        self.arena_id_vec.push(arena_id);
        self.lang_vec.push(lang.into());
        self.mtgo_id_vec.push(mtgo_id);
        self.mtgo_foil_id_vec.push(mtgo_foil_id);
        self.multiverse_ids_vec.push(multiverse_ids);
        self.resource_id_vec.push(resource_id);
        self.tcgplayer_id_vec.push(tcgplayer_id);
        self.tcgplayer_etched_id_vec.push(tcgplayer_etched_id);
        self.cardmarket_id_vec.push(cardmarket_id);
        self.layout_vec.push(layout.into());

    }

    fn into_dataframe(self) -> PolarsResult<DataFrame> {
        let lang_series = ChunkedArray::from_iter(self.lang_vec)
            .cast(&SCRYFALL_SCHEMA.get_field("language").unwrap().dtype)?;

        let layout_series = ChunkedArray::from_iter(self.layout_vec)
            .cast(&SCRYFALL_SCHEMA.get_field("layout").unwrap().dtype)?;

        polars::df! {
            "arena_id" => self.arena_id_vec,
            "id" => self.id_vec,
            "language" => lang_series,
            "mtgo_id" => self.mtgo_id_vec,
            "mtgo_foil_id" => self.mtgo_foil_id_vec,

            "multiverse_ids" => self.multiverse_ids_vec
                .into_iter()
                .map(|ids| match ids {
                    None => AnyValue::Null,
                    Some(list) => AnyValue::List(
                        Series::from_iter(list)
                    )
                })
                .collect::<Vec<_>>(),

            "resource_id" => self.resource_id_vec,
            "tcgplayer_id" => self.tcgplayer_id_vec,
            "tcgplayer_etched_id" => self.tcgplayer_etched_id_vec,
            "cardmarket_id" => self.cardmarket_id_vec,
            "layout" => layout_series,

        }
    }
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

            let wrapped_reader = ReadWrapper {
                read_bytes: 0,
                cb: wrapper_cb,
                reader: response,
            };

            let mut json_reader = JsonStreamReader::new(wrapped_reader);

            json_reader.begin_array()?;

            let mut collector = ColumnHolder::default();

            while json_reader.has_next()? {
                let card = json_reader.deserialize_next::<ScryfallCard>()?;
                collector.push(card);
                cards_transformed.fetch_add(1, Ordering::Relaxed);
            }

            json_reader.end_array()?;
            let mut df = collector.into_dataframe()?;

            assert_eq!(
                df.schema(),
                &*SCRYFALL_SCHEMA,
                "dataframe does not match desired schema"
            );

            let mut file = File::create(&*SCRYFALL_DATA_FILE_PATH)?;
            ParquetWriter::new(&mut file).finish(&mut df)?;
            Ok(())
        }).await?
    }
}
