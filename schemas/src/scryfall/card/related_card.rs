use serde::{Deserialize, Deserializer, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;
use url::Url;
use uuid::Uuid;

#[derive(
    Deserialize,
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    TypeName,
    EnumIter,
    IntoStaticStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
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
