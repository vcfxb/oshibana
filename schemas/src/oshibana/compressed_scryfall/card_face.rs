use std::collections::HashMap;
use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;
use crate::oshibana::compressed_scryfall::CompressedUuid;
use crate::scryfall::card::colors::Color;
use crate::scryfall::card::layout::Layout;
use crate::utils::compressed_enum::AsDiscriminant;

#[derive(Deserialize, Debug, Serialize)]
pub struct CompressedCardFace {
    pub artist: Option<String>,
    pub artist_id: Option<CompressedUuid>,
    pub cmc: Option<f32>,
    pub color_indicator: Option<BitFlags<Color>>,
    pub colors: Option<BitFlags<Color>>,

    pub defense: Option<String>,

    pub flavor_text: Option<String>,
    pub illustration_id: Option<CompressedUuid>,

    pub image_uris: Option<HashMap<String, Url>>,
    pub layout: Option<AsDiscriminant<Layout>>,

    pub loyalty: Option<String>,

    pub mana_cost: String,

    pub name: String,
    pub oracle_id: Option<CompressedUuid>,

    pub oracle_text: Option<String>,

    pub power: Option<String>,

    pub printed_name: Option<String>,

    pub printed_text: Option<String>,

    pub printed_type_line: Option<String>,

    pub toughness: Option<String>,

    pub type_line: Option<String>,

    pub watermark: Option<String>,
}
