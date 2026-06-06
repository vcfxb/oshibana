//! Oshibana schema, including oshibana compressions of scryfall data stored on disc.

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::oshibana::collection::Collection;
use crate::oshibana::deck::Deck;
use crate::oshibana::package::Package;
use crate::oshibana::wishlist::WishlistItem;

pub mod collection;
pub mod deck;
pub mod package;
pub mod wishlist;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserData {
    pub last_scryfall_sync: Option<DateTime<Utc>>,
    #[serde(default)]
    pub scryfall_sync_interval: Option<chrono::Duration>,
    pub decks: Vec<Deck>,
    pub collection: Collection,
    pub wishlist: Vec<WishlistItem>,
    pub packages: Vec<Package>,
    /// Apply global oracle tags in wishlist, decks, & collection
    pub global_oracle_tags: HashMap<Uuid, Vec<String>>,
    #[serde(default)] // todo: better default here
    pub visible_search_columns: Vec<String>,
}
// fn default_visible_columns() -> Vec<String> {
//     [
//         "name",
//         "mana_cost",
//         "type_line",
//         "rarity",
//         "set",
//         "collector_number",
//         "lang",
//     ]
//     .map(ToOwned::to_owned)
//     .to_vec()
// }


impl Default for UserData {
    fn default() -> Self {
        UserData {
            last_scryfall_sync: None,
            scryfall_sync_interval: None,
            decks: vec![],
            collection: Default::default(),
            wishlist: vec![],
            packages: vec![],
            global_oracle_tags: Default::default(),
            visible_search_columns: Vec::new(),
        }
    }
}
