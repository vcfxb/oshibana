use crate::scryfall::card::colors::Color;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct CardSymbol {
    pub symbol: String,
    pub loose_variant: Option<String>,
    pub english: String,
    pub transposable: bool,
    pub represents_mana: bool,
    pub mana_value: Option<f32>,
    pub appears_in_mana_costs: bool,
    pub funny: bool,
    pub colors: Vec<Color>,
    pub hybrid: bool,
    pub phyrexian: bool,
    // skip gatherer_alternatives, we don't use it.
    pub svg_uri: Option<Url>,
}
