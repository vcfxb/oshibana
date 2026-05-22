use crate::generate_record_builder_and_dt;
use crate::scryfall::card::card_face::CardFace;
use crate::scryfall::card::colors::Color;
use crate::scryfall::card::finishes::Finish;
use crate::scryfall::card::frame::FrameEffect;
use crate::scryfall::card::games::Game;
use crate::scryfall::card::image_status::ImageStatus;
use crate::scryfall::card::image_uris::ImageUris;
use crate::scryfall::card::languages::Language;
use crate::scryfall::card::layout::Layout;
use crate::scryfall::card::legalities::Legalities;
use crate::scryfall::card::prices::Prices;
use crate::scryfall::card::purchase_uris::PurchaseUris;
use crate::scryfall::card::rarity::Rarity;
use crate::scryfall::card::related_card::RelatedCard;
use crate::scryfall::card::related_uris::RelatedUris;
use crate::scryfall::card::security_stamp::SecurityStamp;
use crate::scryfall::set::SetType;
use crate::traits::builder::PolarsBuilder;
use chrono::{DateTime, Utc};
use polars::chunked_array::StructChunked;
use polars::prelude::{DataFrame, PolarsResult};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

mod card_face;
mod colors;
mod finishes;
mod frame;
mod games;
mod image_status;
mod image_uris;
mod languages;
mod layout;
mod legalities;
mod prices;
mod purchase_uris;
mod rarity;
mod related_card;
mod related_uris;
mod security_stamp;

generate_record_builder_and_dt! {
    #[derive(Debug, Deserialize, Serialize)]
    ScryfallCard {
        arena_id: Option<u64>,
        id: Uuid,
        lang: Language,
        mtgo_id: Option<u64>,
        mtgo_foil_id: Option<u64>,
        multiverse_ids: Option<Vec<u64>>,
        resource_id: Option<String>,
        tcgplayer_id: Option<u64>,
        tcgplayer_etched_id: Option<u64>,
        cardmarket_id: Option<u64>,
        layout: Layout,
        oracle_id: Option<Uuid>,
        prints_search_uri: Url,
        rulings_uri: Url,
        scryfall_uri: Url,
        uri: Url,
        all_parts: Option<Vec<RelatedCard>>,
        card_faces: Option<Vec<CardFace>>,
        cmc: f32,
        color_identity: Vec<Color>,
        color_indicator: Option<Vec<Color>>,
        colors: Option<Vec<Color>>,

        defense: Option<String>,
        edhrec_rank: Option<u64>,
        game_changer: Option<bool>,

        hand_modifier: Option<String>,

        keywords: Vec<String>,

        legalities: Legalities,

        life_modifier: Option<String>,
        loyalty: Option<String>,
        mana_cost: Option<String>,
        name: String,
        oracle_text: Option<String>,
        penny_rank: Option<u64>,

        power: Option<String>,
        produced_mana: Option<Vec<Color>>,
        reserved: bool,
        toughness: Option<String>,
        type_line: String,

        artist: Option<String>,
        artist_ids: Option<Vec<Uuid>>,
        attraction_lights: Option<Vec<u8>>,
        booster: bool,
        border_color: String,
        card_back_id: Uuid,
        collector_number: String,
        content_warning: Option<bool>,
        digital: bool,
        finishes: Vec<Finish>,
        flavor_name: Option<String>,
        flavor_text: Option<String>,
        frame_effects: Option<Vec<FrameEffect>>,
        frame: String,
        full_art: bool,
        games: Vec<Game>,
        highres_image: bool,
        illustration_id: Option<Uuid>,
        image_status: ImageStatus,

        image_uris: Option<ImageUris>,
        oversized: bool,
        prices: Prices,
        printed_name: Option<String>,
        printed_text: Option<String>,
        printed_type_line: Option<String>,
        promo: bool,
        promo_types: Option<Vec<String>>,
        purchase_uris: Option<PurchaseUris>,
        rarity: Rarity,

        related_uris: RelatedUris,
        released_at: DateTime<Utc>,
        reprint: bool,
        scryfall_set_uri: Url,

        set_name: String,
        set_search_uri: Url,
        set_type: SetType,
        set_uri: Url,

        set: String,
        set_id: Uuid,
        story_spotlight: bool,
        textless: bool,
        variation: bool,
        variation_of: Option<Uuid>,
        security_stamp: Option<SecurityStamp>,

        watermark: Option<String>,
    }
}

impl ScryfallCardBuilder {
    /// Finishes this builder and then breaks out the fields into columns of a [`DataFrame`].
    pub fn finish_into_dataframe(self) -> PolarsResult<DataFrame> {
        let chunked: StructChunked = PolarsBuilder::<ScryfallCard>::finish(self)?;
        let cols = chunked.fields_as_columns();
        DataFrame::new_infer_height(cols)
    }
}
