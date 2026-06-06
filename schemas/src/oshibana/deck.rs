// Decks should appear with a mainboard, sideboard, and consideringboard by default, as well
// as commander board for commander decks (make sure works for partner/background/etc)
// also maybe companion board?

/*
Deck builder views:
- working / current / side-by-side diff / inline-diff
- cards-as-text etc (moxfield)
- highlighter: filter by board
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Deck {
    pub name: String,
    /// Empty string for none
    pub description: String,
    /// Empty string for none
    pub primer: String,
    pub cards: Vec<DeckCard>,
    pub tags: HashMap<Uuid, Vec<String>>,
    pub history: Vec<DeckAction>,
    pub format: Format,
    pub listed_bracket: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Format {
    Commander,
    Standard,
    None,
    Modern,
    Cube,
    Pauper,
    Vintage,
    Legacy,
    PauperCommander,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DeckCard {
    pub quantity: u32,
    pub card_id: Uuid,
    pub board: String,
    pub is_foil: bool,
    pub is_etched: bool,
    // Overrides should be toggle-able
    pub mana_cost_override: Option<String>,
    pub name_override: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeckAction {
    pub quantity_delta: i32,
    pub card_id: Uuid,
    pub is_foil: bool,
    pub is_etched: bool,
    pub board: String,
}
