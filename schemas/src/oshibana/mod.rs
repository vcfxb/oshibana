//! Oshibana schema, including oshibana compressions of scryfall data stored on disc.

use crate::oshibana::collection::Collection;
use crate::oshibana::deck::Deck;
use crate::oshibana::package::Package;
use crate::oshibana::wishlist::WishlistItem;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::{Display, EnumIter};
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

    #[serde(default = "SearchViewColumn::defaults")]
    pub visible_search_columns: Vec<SearchViewColumn>,

    #[serde(default)]
    pub search_sort_by: SortBy,

    /// Prefix automatically placed before search queries
    #[serde(default)]
    pub search_prefix: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Copy, Clone, Display, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum SearchViewColumn {
    Name,
    Type,
    #[strum(to_string = "Mana Cost")]
    ManaCost,
}

impl SearchViewColumn {
    fn defaults() -> Vec<SearchViewColumn> {
        use SearchViewColumn::*;
        vec![Name, Type]
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
            visible_search_columns: SearchViewColumn::defaults(),
            search_prefix: Default::default(),
            search_sort_by: Default::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Display, EnumIter)]
pub enum UniqueBy {
    Printings,

    #[default]
    Cards,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Display)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Ascending,
    Descending,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Display)]
#[serde(rename_all = "snake_case")]
pub enum SortBy {
    #[strum(to_string = "Name ({0})")]
    Name(Direction),

    #[strum(to_string = "Release Date ({0})")]
    ReleaseDate(Direction),
}

impl Default for SortBy {
    fn default() -> Self {
        SortBy::Name(Direction::Ascending)
    }
}

impl SortBy {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            SortBy::Name(Direction::Ascending),
            SortBy::Name(Direction::Descending),
            SortBy::ReleaseDate(Direction::Ascending),
            SortBy::ReleaseDate(Direction::Descending),
        ]
        .into_iter()
    }
}
