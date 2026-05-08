//! Compressed version of [crate::scryfall::card::Card] and subtypes.

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};
use url::Url;
use crate::oshibana::compressed_scryfall::card_face::CompressedCardFace;
use crate::oshibana::compressed_scryfall::CompressedUuid;
use crate::oshibana::compressed_scryfall::related_card::CompressedRelatedCard;
use crate::scryfall::card::colors::Color;
use crate::scryfall::card::finishes::Finish;
use crate::scryfall::card::frame::FrameEffect;
use crate::scryfall::card::games::Game;
use crate::scryfall::card::image_status::ImageStatus;
use crate::scryfall::card::languages::Language;
use crate::scryfall::card::layout::Layout;
use crate::scryfall::card::legalities::Legality;
use crate::scryfall::card::rarity::Rarity;
use crate::scryfall::card::security_stamp::SecurityStamp;
use crate::scryfall::set::SetType;
use crate::utils::compressed_enum::AsDiscriminant;

#[derive(Serialize, Deserialize)]
pub struct CompressedScryfallCard {
    pub arena_id: Option<u64>,
    pub id: CompressedUuid,
    pub lang: AsDiscriminant<Language>,
    pub mtgo_id: Option<u64>,
    pub mtgo_foil_id: Option<u64>,
    pub multiverse_ids: Option<Vec<u64>>,

    /// Gatherer resource ID
    pub resource_id: Option<String>,

    /*
    pub tcgplayer_id: Option<u64>,
    pub tcgplayer_etched_id: Option<u64>,
    pub cardmarket_id: Option<u64>,
    */

    pub layout: AsDiscriminant<Layout>,
    pub oracle_id: Option<CompressedUuid>,
    pub prints_search_uri: Url,
    pub rulings_uri: Url,
    pub scryfall_uri: Url,
    pub uri: Url,

    pub all_parts: Option<Vec<CompressedRelatedCard>>,

    pub card_faces: Option<Vec<CompressedCardFace>>,
    pub cmc: f32,
    pub color_identity: BitFlags<Color>,
    pub color_indicator: Option<BitFlags<Color>>,
    pub colors: Option<BitFlags<Color>>,

    pub defense: Option<String>,
    pub edhrec_rank: Option<u64>,
    pub game_changer: Option<bool>,

    pub hand_modifier: Option<String>,

    pub keywords: Vec<String>,

    pub legalities: HashMap<String, AsDiscriminant<Legality>>,

    pub life_modifier: Option<String>,

    pub loyalty: Option<String>,

    pub mana_cost: Option<String>,

    pub name: String,

    pub oracle_text: Option<String>,
    pub penny_rank: Option<u64>,

    pub power: Option<String>,
    pub produced_mana: Option<BitFlags<Color>>,
    pub reserved: bool,

    pub toughness: Option<String>,

    pub type_line: String,

    pub artist: Option<String>,
    pub artist_ids: Option<Vec<CompressedUuid>>,
    pub attraction_lights: Option<Vec<u8>>,
    pub booster: bool,

    pub border_color: String,
    pub card_back_id: CompressedUuid,

    pub collector_number: String,
    pub content_warning: Option<bool>,
    pub digital: bool,
    pub finishes: BitFlags<Finish>,

    pub flavor_name: Option<String>,

    pub flavor_text: Option<String>,
    pub frame_effects: Option<BitFlags<FrameEffect>>,

    pub frame: String,
    pub full_art: bool,
    pub games: BitFlags<Game>,
    pub highres_image: bool,
    pub illustration_id: Option<CompressedUuid>,
    pub image_status: AsDiscriminant<ImageStatus>,

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
    pub set_type: AsDiscriminant<SetType>,
    pub set_uri: Url,

    pub set: String,
    pub set_id: CompressedUuid,
    pub story_spotlight: bool,
    pub textless: bool,
    pub variation: bool,
    pub variation_of: Option<CompressedUuid>,
    pub security_stamp: Option<AsDiscriminant<SecurityStamp>>,

    pub watermark: Option<String>,
}
