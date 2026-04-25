use crate::utils::array_to_bitflags::ArrayToBitset;
use crate::utils::deserialize_matches::DeserializeMatches;
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;
use crate::scryfall::card::colors::Color;
use crate::scryfall::card::layout::Layout;

#[derive(Deserialize, Debug)]
pub struct CardFace<'a> {
    #[serde(borrow)]
    pub artist: Option<Cow<'a, str>>,
    pub artist_id: Option<Uuid>,
    pub cmc: Option<f32>,
    pub color_indicator: Option<ArrayToBitset<Color>>,
    pub colors: Option<ArrayToBitset<Color>>,
    #[serde(borrow)]
    pub defense: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub flavor_text: Option<Cow<'a, str>>,
    pub illustration_id: Option<Uuid>,
    #[serde(borrow)]
    pub image_uris: Option<HashMap<Cow<'a, str>, Url>>,
    pub layout: Option<Layout>,
    #[serde(borrow)]
    pub loyalty: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub mana_cost: Cow<'a, str>,
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(deserialize_with = "deserialize_object_name")]
    pub object: &'static str,
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

fn deserialize_object_name<'de, D>(deserializer: D) -> Result<&'static str, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(DeserializeMatches("card_face"))
}
