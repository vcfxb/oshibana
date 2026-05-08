//! Scryfall data that oshibana stores on the user's computer

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::oshibana::compressed_scryfall::card::CompressedScryfallCard;
use crate::oshibana::compressed_scryfall::set::CompressedScryfallSet;

pub mod card;
pub mod card_face;
pub mod related_card;
pub mod set;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompressedUuid(#[serde(with = "uuid::serde::compact")] pub Uuid);

#[derive(Serialize, Deserialize)]
pub struct ScryfallDataCache {
    pub date_updated: DateTime<Utc>,
    pub all_cards: Vec<CompressedScryfallCard>,
    pub all_sets: Vec<CompressedScryfallSet>
}
