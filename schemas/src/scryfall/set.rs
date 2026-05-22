use crate::generate_enum_dt_map_and_builder_impl;
use chrono::{DateTime, Utc};
use enumflags2::bitflags;
use polars::prelude::Categorical8Type;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;
use url::Url;
use uuid::Uuid;

#[bitflags]
#[derive(
    Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypeName, IntoStaticStr, EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[repr(u32)]
pub enum SetType {
    Core,
    Expansion,
    Masters,
    Eternal,
    Alchemy,
    Masterpiece,
    Arsenal,
    FromTheVault,
    Spellbook,
    PremiumDeck,
    DuelDeck,
    DraftInnovation,
    TreasureChest,
    Commander,
    Planechase,
    Archenemy,
    Vanguard,
    Funny,
    Starter,
    Box,
    Promo,
    Token,
    Memorabilia,
    Minigame,
}

generate_enum_dt_map_and_builder_impl!(SetType => Categorical8Type);

#[derive(Deserialize, Debug)]
pub struct ScryfallSet {
    pub id: Uuid,
    pub code: String,
    pub mtgo_code: Option<String>,
    pub arena_code: Option<String>,
    pub tcgplayer_id: Option<i64>,
    pub name: String,
    pub set_type: SetType,
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
