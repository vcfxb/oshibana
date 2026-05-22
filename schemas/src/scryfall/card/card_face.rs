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

        // #[serde(borrow)]
        defense: Option<String>,

        // #[serde(borrow)]
        flavor_text: Option<String>,
        illustration_id: Option<Uuid>,
        
        // #[serde(borrow)]
        image_uris: Option<ImageUris>,
        layout: Option<Layout>,

    }
}
#[derive(Deserialize, Debug, Serialize)]
pub struct CardFaceOld<'a> {

    
    #[serde(borrow)]
    pub loyalty: Option<Cow<'a, str>>,

    #[serde(borrow)]
    pub mana_cost: Cow<'a, str>,

    #[serde(borrow)]
    pub name: Cow<'a, str>,

    pub oracle_id: Option<Uuid>,

    #[serde(borrow)]
    pub oracle_text: Option<Cow<'a, str>>,

    #[serde(borrow)]
    pub power: Option<Cow<'a, str>>,

    #[serde(borrow)]
    pub printed_name: Option<Cow<'a, str>>,

    #[serde(borrow)]
    pub printed_text: Option<Cow<'a, str>>,

    #[serde(borrow)]
    pub printed_type_line: Option<Cow<'a, str>>,

    #[serde(borrow)]
    pub toughness: Option<Cow<'a, str>>,

    #[serde(borrow)]
    pub type_line: Option<Cow<'a, str>>,

    #[serde(borrow)]
    pub watermark: Option<Cow<'a, str>>,
}
