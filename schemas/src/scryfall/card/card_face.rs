use crate::utils::array_to_bitflags::ArrayToBitset;
use crate::utils::deserialize_matches::DeserializeMatches;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;
use crate::scryfall::card::colors::Color;
use crate::scryfall::card::layout::Layout;

#[derive(Deserialize, Debug, Serialize)]
pub struct CardFace {
    
    pub artist: Option<String>,
    pub artist_id: Option<Uuid>,
    pub cmc: Option<f32>,
    pub color_indicator: Option<ArrayToBitset<Color>>,
    pub colors: Option<ArrayToBitset<Color>>,
    
    pub defense: Option<String>,
    
    pub flavor_text: Option<String>,
    pub illustration_id: Option<Uuid>,
    
    pub image_uris: Option<HashMap<String, Url>>,
    pub layout: Option<Layout>,
    
    pub loyalty: Option<String>,
    
    pub mana_cost: String,
    
    pub name: String,
    #[serde(deserialize_with = "deserialize_object_name")]
    pub object: &'static str,
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

fn deserialize_object_name<'de, D>(deserializer: D) -> Result<&'static str, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(DeserializeMatches("card_face"))
}
