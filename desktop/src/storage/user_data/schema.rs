use crate::storage::user_data::collection::Collection;
use crate::storage::user_data::deck::Deck;
use crate::storage::user_data::package::Package;
use crate::storage::user_data::wishlist::WishlistItem;
use chrono::{DateTime, Utc};
use egui::accesskit::Uuid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    #[serde(default = "default_visible_columns")]
    pub visible_search_columns: Vec<String>,
}

fn default_visible_columns() -> Vec<String> {
    [
        "name",
        "mana_cost",
        "type_line",
        "rarity",
        "set",
        "collector_number",
        "lang",
    ].map(ToOwned::to_owned).to_vec()
}

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
            visible_search_columns: default_visible_columns(),
        }
    }
}
