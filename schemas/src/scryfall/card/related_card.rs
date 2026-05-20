use std::borrow::Cow;
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
pub struct RelatedCard<'a> {
    pub id: Uuid,
    pub component: Component,
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(borrow)]
    pub type_line: Cow<'a, str>,
    pub uri: Url,
}
