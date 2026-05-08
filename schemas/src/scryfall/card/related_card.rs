use enum_ordinalize::Ordinalize;
use crate::utils::deserialize_matches::DeserializeMatches;
use serde::{Deserialize, Deserializer, Serialize};
use typename::TypeName;
use url::Url;
use uuid::Uuid;

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, TypeName, Ordinalize)]
#[serde(rename_all = "snake_case")]
pub enum Component {
    Token,
    MeldPart,
    MeldResult,
    ComboPiece,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct RelatedCard {
    pub id: Uuid,
    pub component: Component,
    pub name: String,
    pub type_line: String,
    pub uri: Url,
}

fn deserialize_object_name<'de, D>(deserializer: D) -> Result<&'static str, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(DeserializeMatches("related_card"))
}
