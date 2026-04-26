use crate::utils::array_to_bitflags::ArrayToBitset;
use crate::utils::deserialize_matches::DeserializeMatches;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;
use crate::scryfall::card::card_face::CardFace;
use crate::scryfall::card::colors::Color;
use crate::scryfall::card::finishes::Finish;
use crate::scryfall::card::frame::FrameEffect;
use crate::scryfall::card::games::Game;
use crate::scryfall::card::image_status::ImageStatus;
use crate::scryfall::card::languages::Language;
use crate::scryfall::card::layout::Layout;
use crate::scryfall::card::legalities::Legality;
use crate::scryfall::card::rarity::Rarity;
use crate::scryfall::card::related_card::RelatedCard;
use crate::scryfall::card::security_stamp::SecurityStamp;
use crate::scryfall::set::SetType;

pub mod card_face;
pub mod colors;
pub mod finishes;
pub mod frame;
pub mod games;
pub mod image_status;
pub mod languages;
pub mod layout;
pub mod legalities;
pub mod rarity;
pub mod related_card;
pub mod security_stamp;

#[derive(Debug, Deserialize, Serialize)]
pub struct ScryfallCard<'a> {
    pub arena_id: Option<u64>,
    pub id: Uuid,
    pub lang: Language,
    pub mtgo_id: Option<u64>,
    pub mtgo_foil_id: Option<u64>,
    pub multiverse_ids: Option<Vec<u64>>,
    #[serde(borrow)]
    pub resource_id: Option<Cow<'a, str>>,
    pub tcgplayer_id: Option<u64>,
    pub tcgplayer_etched_id: Option<u64>,
    pub cardmarket_id: Option<u64>,
    #[serde(deserialize_with = "deserialize_object_name")]
    pub object: &'static str,
    pub layout: Layout,
    pub oracle_id: Option<Uuid>,
    pub prints_search_uri: Url,
    pub rulings_uri: Url,
    pub scryfall_uri: Url,
    pub uri: Url,

    #[serde(borrow)]
    pub all_parts: Option<Vec<RelatedCard<'a>>>,
    #[serde(borrow)]
    pub card_faces: Option<Vec<CardFace<'a>>>,
    pub cmc: f32,
    pub color_identity: ArrayToBitset<Color>,
    pub color_indicator: Option<ArrayToBitset<Color>>,
    pub colors: Option<ArrayToBitset<Color>>,
    #[serde(borrow)]
    pub defense: Option<Cow<'a, str>>,
    pub edhrec_rank: Option<u64>,
    pub game_changer: Option<bool>,
    #[serde(borrow)]
    pub hand_modifier: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub keywords: Vec<Cow<'a, str>>,
    #[serde(borrow)]
    pub legalities: HashMap<Cow<'a, str>, Legality>,
    #[serde(borrow)]
    pub life_modifier: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub loyalty: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub mana_cost: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(borrow)]
    pub oracle_text: Option<Cow<'a, str>>,
    pub penny_rank: Option<u64>,
    #[serde(borrow)]
    pub power: Option<Cow<'a, str>>,
    pub produced_mana: Option<ArrayToBitset<Color>>,
    pub reserved: bool,
    #[serde(borrow)]
    pub toughness: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub type_line: Cow<'a, str>,

    #[serde(borrow)]
    pub artist: Option<Cow<'a, str>>,
    pub artist_ids: Option<Vec<Uuid>>,
    pub attraction_lights: Option<Vec<u8>>,
    pub booster: bool,
    #[serde(borrow)]
    pub border_color: Cow<'a, str>,
    pub card_back_id: Uuid,
    #[serde(borrow)]
    pub collector_number: Cow<'a, str>,
    pub content_warning: Option<bool>,
    pub digital: bool,
    pub finishes: Vec<Finish>,
    #[serde(borrow)]
    pub flavor_name: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub flavor_text: Option<Cow<'a, str>>,
    pub frame_effects: Option<ArrayToBitset<FrameEffect>>,
    #[serde(borrow)]
    pub frame: Cow<'a, str>,
    pub full_art: bool,
    pub games: ArrayToBitset<Game>,
    pub highres_image: bool,
    pub illustration_id: Option<Uuid>,
    pub image_status: ImageStatus,
    #[serde(borrow)]
    pub image_uris: Option<HashMap<Cow<'a, str>, Url>>,
    pub oversized: bool,
    #[serde(borrow)]
    pub prices: HashMap<Cow<'a, str>, f32>,
    #[serde(borrow)]
    pub printed_name: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub printed_text: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub printed_type_line: Option<Cow<'a, str>>,
    pub promo: bool,
    #[serde(borrow)]
    pub promo_types: Option<Vec<Cow<'a, str>>>,
    #[serde(borrow)]
    pub purchase_uris: Option<HashMap<Cow<'a, str>, Url>>,
    pub rarity: Rarity,
    #[serde(borrow)]
    pub related_uris: HashMap<Cow<'a, str>, Url>,
    pub released_at: DateTime<Utc>,
    pub reprint: bool,
    pub scryfall_set_uri: Url,
    #[serde(borrow)]
    pub set_name: Cow<'a, str>,
    pub set_search_uri: Url,
    pub set_type: SetType,
    pub set_uri: Url,
    #[serde(borrow)]
    pub set: Cow<'a, str>,
    pub set_id: Uuid,
    pub story_spotlight: bool,
    pub textless: bool,
    pub variation: bool,
    pub variation_of: Option<Uuid>,
    pub security_stamp: Option<SecurityStamp>,
    #[serde(borrow)]    
    pub watermark: Option<Cow<'a, str>>,
}

fn deserialize_object_name<'de, D>(deserializer: D) -> Result<&'static str, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(DeserializeMatches("card"))
}
