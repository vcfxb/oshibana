use egui::accesskit::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    pub name: String,
    /// Can be empty
    pub description: String,
    pub oracle_cards: Vec<Uuid>,
}
