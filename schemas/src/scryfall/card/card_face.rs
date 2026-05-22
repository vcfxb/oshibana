use std::borrow::Cow;
use crate::scryfall::card::colors::Color;
use crate::scryfall::card::layout::Layout;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;
use crate::generate_record_builder_and_dt;
use crate::scryfall::card::image_uris::ImageUris;

generate_record_builder_and_dt! {
    #[derive(Deserialize, Debug, Serialize)]
    CardFace {
        artist: Option<String>,
        artist_id: Option<Uuid>,
        cmc: Option<f32>,
        color_indicator: Option<Vec<Color>>,
        colors: Option<Vec<Color>>,
        defense: Option<String>,
        flavor_text: Option<String>,
        illustration_id: Option<Uuid>,
        image_uris: Option<ImageUris>,
        layout: Option<Layout>,
        loyalty: Option<String>,
        mana_cost: String,
        name: String,
        oracle_id: Option<Uuid>,
        oracle_text: Option<String>,
        power: Option<String>,
        printed_name: Option<String>,
        printed_text: Option<String>,
        printed_type_line: Option<String>,
        toughness: Option<String>,
        type_line: Option<String>,
        watermark: Option<String>,
    }
}
