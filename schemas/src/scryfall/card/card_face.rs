use crate::scryfall::card::colors::Color;
use crate::scryfall::card::layout::Layout;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

#[derive(Deserialize, Debug, Serialize)]
pub struct CardFace {
    pub artist: Option<String>,
    pub artist_id: Option<Uuid>,
    pub cmc: Option<f32>,
    pub color_indicator: Option<Vec<Color>>,
    pub colors: Option<Vec<Color>>,

    pub defense: Option<String>,

    pub flavor_text: Option<String>,
    pub illustration_id: Option<Uuid>,

    pub image_uris: Option<HashMap<String, Url>>,
    pub layout: Option<Layout>,

    pub loyalty: Option<String>,

    pub mana_cost: String,

    pub name: String,

    pub oracle_id: Option<Uuid>,

    pub oracle_text: Option<String>,

    pub power: Option<String>,

    pub printed_name: Option<String>,

    pub printed_text: Option<String>,

    pub printed_type_line: Option<String>,

    pub toughness: Option<String>,

    pub type_line: Option<String>,

    pub watermark: Option<String>,
}
