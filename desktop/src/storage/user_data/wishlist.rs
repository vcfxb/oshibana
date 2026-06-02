use serde::{Deserialize, Serialize};
use egui::accesskit::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WishlistItem {
    pub card_id: Uuid,
    pub tags: Vec<String>,
    pub is_foil: bool,
    pub is_etched: bool,
}