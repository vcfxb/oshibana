use crate::utils::deserialize_matches::DeserializeMatches;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;
use url::Url;
use uuid::Uuid;

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
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

#[derive(Deserialize, Debug)]
pub struct ScryfallSet<'a> {
    #[serde(deserialize_with = "deserialize_object_name")]
    pub object: &'static str,
    pub id: Uuid,
    #[serde(borrow)]
    pub code: Cow<'a, str>,
    #[serde(borrow)]
    pub mtgo_code: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub arena_code: Option<Cow<'a, str>>,
    pub tcgplayer_id: Option<i64>,
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    pub set_type: SetType,
    pub released_at: Option<DateTime<Utc>>,
    #[serde(borrow)]
    pub block_code: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub block: Option<Cow<'a, str>>,
    #[serde(borrow)]
    pub parent_set_code: Option<Cow<'a, str>>,
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

fn deserialize_object_name<'de, D>(deserializer: D) -> Result<&'static str, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(DeserializeMatches("set"))
}
