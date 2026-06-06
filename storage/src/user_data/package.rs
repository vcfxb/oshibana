use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    pub name: String,
    /// Can be empty
    pub description: String,
    pub oracle_cards: Vec<Uuid>,
}
