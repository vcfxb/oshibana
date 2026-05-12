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
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

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
pub struct ScryfallCard {
    pub arena_id: Option<u64>,
    pub id: Uuid,
    pub lang: Language,
    pub mtgo_id: Option<u64>,
    pub mtgo_foil_id: Option<u64>,
    pub multiverse_ids: Option<Vec<u64>>,

    pub resource_id: Option<String>,
    pub tcgplayer_id: Option<u64>,
    pub tcgplayer_etched_id: Option<u64>,
    pub cardmarket_id: Option<u64>,

    pub layout: Layout,
    pub oracle_id: Option<Uuid>,
    pub prints_search_uri: Url,
    pub rulings_uri: Url,
    pub scryfall_uri: Url,
    pub uri: Url,

    pub all_parts: Option<Vec<RelatedCard>>,

    pub card_faces: Option<Vec<CardFace>>,
    pub cmc: f32,
    pub color_identity: Vec<Color>,
    pub color_indicator: Option<Vec<Color>>,
    pub colors: Option<Vec<Color>>,

    pub defense: Option<String>,
    pub edhrec_rank: Option<u64>,
    pub game_changer: Option<bool>,

    pub hand_modifier: Option<String>,

    pub keywords: Vec<String>,

    pub legalities: HashMap<String, Legality>,

    pub life_modifier: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub name: String,
    pub oracle_text: Option<String>,
    pub penny_rank: Option<u64>,

    pub power: Option<String>,
    pub produced_mana: Option<Vec<Color>>,
    pub reserved: bool,
    pub toughness: Option<String>,
    pub type_line: String,

    pub artist: Option<String>,
    pub artist_ids: Option<Vec<Uuid>>,
    pub attraction_lights: Option<Vec<u8>>,
    pub booster: bool,
    pub border_color: String,
    pub card_back_id: Uuid,
    pub collector_number: String,
    pub content_warning: Option<bool>,
    pub digital: bool,
    pub finishes: Vec<Finish>,
    pub flavor_name: Option<String>,
    pub flavor_text: Option<String>,
    pub frame_effects: Option<Vec<FrameEffect>>,
    pub frame: String,
    pub full_art: bool,
    pub games: Vec<Game>,
    pub highres_image: bool,
    pub illustration_id: Option<Uuid>,
    pub image_status: ImageStatus,

    pub image_uris: Option<HashMap<String, Url>>,
    pub oversized: bool,

    pub prices: HashMap<String, f32>,

    pub printed_name: Option<String>,

    pub printed_text: Option<String>,

    pub printed_type_line: Option<String>,
    pub promo: bool,

    pub promo_types: Option<Vec<String>>,

    pub purchase_uris: Option<HashMap<String, Url>>,
    pub rarity: Rarity,

    pub related_uris: HashMap<String, Url>,
    pub released_at: DateTime<Utc>,
    pub reprint: bool,
    pub scryfall_set_uri: Url,

    pub set_name: String,
    pub set_search_uri: Url,
    pub set_type: SetType,
    pub set_uri: Url,

    pub set: String,
    pub set_id: Uuid,
    pub story_spotlight: bool,
    pub textless: bool,
    pub variation: bool,
    pub variation_of: Option<Uuid>,
    pub security_stamp: Option<SecurityStamp>,

    pub watermark: Option<String>,
}
