use serde::{Deserialize, Serialize};
use url::Url;
use crate::oshibana::compressed_scryfall::CompressedUuid;
use crate::scryfall::card::related_card::Component;
use crate::utils::compressed_enum::AsDiscriminant;

#[derive(Deserialize, Debug, Serialize)]
pub struct CompressedRelatedCard {
    pub id: CompressedUuid,
    pub component: AsDiscriminant<Component>,
    pub name: String,
    pub type_line: String,
    pub uri: Url,
}
