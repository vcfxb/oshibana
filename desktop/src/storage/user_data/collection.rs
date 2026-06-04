use chrono::{DateTime, Utc};
use egui::accesskit::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct Collection {
    pub storage_locations: Vec<StorageLocation>,
    pub cards: Vec<CollectionCard>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectionCard {
    pub quantity: u32,
    pub card_id: Uuid,
    pub condition: Condition,
    pub is_alter: bool,
    pub is_proxy: bool,
    pub is_foil: bool,
    pub is_etched: bool,
    pub date_added: DateTime<Utc>,
    pub date_updated: DateTime<Utc>,
    pub deck_name_assigned: Option<String>,
    pub physical_storage_assigned: Option<String>,
    pub notes: Option<String>,
    /// Can be empty
    pub purchase_price: String,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum Condition {
    Mint,
    NearMint,
    LightlyPlayed,
    MediumPlay,
    HeavilyPlayed,
    Damaged,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StorageLocation {
    pub name: String,
    /// Can be empty.
    pub description: String,
}
