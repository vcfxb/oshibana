//! Compressed scryfall set schema

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;
use crate::oshibana::compressed_scryfall::CompressedUuid;
use crate::scryfall::set::SetType;
use crate::utils::compressed_enum::AsDiscriminant;

#[derive(Deserialize, Serialize, Debug)]
pub struct CompressedScryfallSet {
    pub id: CompressedUuid,
    pub code: String,
    pub mtgo_code: Option<String>,
    pub arena_code: Option<String>,
    pub tcgplayer_id: Option<i64>,
    pub name: String,
    pub set_type: AsDiscriminant<SetType>,
    pub released_at: Option<DateTime<Utc>>,
    pub block_code: Option<String>,
    pub block: Option<String>,
    pub parent_set_code: Option<String>,
    pub card_count: u64,
    pub printed_size: u64,
    pub digital: bool,
    pub foil_only: bool,
    pub nonfoil_only: bool,
    pub scryfall_uri: Url,
    pub uri: Url,
    pub icon_svg_uri: Url,
    pub search_uri: Url,
}
