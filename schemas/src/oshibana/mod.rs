//! Oshibana schema, including oshibana compressions of scryfall data stored on disc.

use crate::oshibana::collection::Collection;
use crate::oshibana::deck::Deck;
use crate::oshibana::package::Package;
use crate::oshibana::wishlist::WishlistItem;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::{EnumIter, IntoStaticStr};
use uuid::Uuid;

pub mod collection;
pub mod deck;
pub mod package;
pub mod wishlist;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserData {
    pub last_scryfall_sync: Option<DateTime<Utc>>,
    #[serde(default)]
    pub scryfall_sync_interval: Option<Duration>,
    #[serde(default)]
    pub autosave_interval: Option<Duration>,
    pub decks: Vec<Deck>,
    pub collection: Collection,
    pub wishlist: Vec<WishlistItem>,
    pub packages: Vec<Package>,

    /// Apply global oracle tags in wishlist, decks, & collection
    pub global_oracle_tags: HashMap<Uuid, Vec<String>>,

    #[serde(default = "SearchColumn::defaults")]
    pub visible_search_columns: Vec<SearchColumn>,

    /// Prefix automatically placed before search queries
    #[serde(default)]
    pub search_prefix: String,
}

#[derive(
    Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Copy, Clone, IntoStaticStr, EnumIter,
)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SearchColumn {
    Name,
    Type,
    ManaCost,
}

impl SearchColumn {
    fn defaults() -> Vec<SearchColumn> {
        use SearchColumn::*;
        vec![Name, Type]
    }

    pub fn into_str(self) -> &'static str {
        self.into()
    }
}

impl Default for UserData {
    fn default() -> Self {
        UserData {
            last_scryfall_sync: None,
            scryfall_sync_interval: None,
            autosave_interval: Some(Duration::seconds(2)),
            decks: vec![],
            collection: Default::default(),
            wishlist: vec![],
            packages: vec![],
            global_oracle_tags: Default::default(),
            visible_search_columns: SearchColumn::defaults(),
            search_prefix: Default::default(),
        }
    }
}
