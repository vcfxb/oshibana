//! Different views that the application uses,

use std::sync::{Arc, Mutex};
use schemas::scryfall::card::ScryfallCard;

pub enum SearchState {
    Idle,
    Searching,
    Results(Vec<ScryfallCard>),
    Error(String),
}

/// Views/pages that a user can be on.
#[derive(Default, PartialEq)]
pub enum View {
    #[default]
    Home,
    Search,
    Collection,
    Decks,
    DeckBom,
    Settings,
}
