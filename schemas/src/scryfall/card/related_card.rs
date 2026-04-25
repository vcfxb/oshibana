use crate::utils::deserialize_matches::DeserializeMatches;
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;
use url::Url;
use uuid::Uuid;

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Component {
    Token,
    MeldPart,
    MeldResult,
    ComboPiece,
}

#[derive(Deserialize, Debug)]
pub struct RelatedCard<'a> {
    pub id: Uuid,
    #[serde(deserialize_with = "deserialize_object_name")]
    pub object: &'static str,
    pub component: Component,
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    #[serde(borrow)]
    pub type_line: Cow<'a, str>,
    pub uri: Url,
}

fn deserialize_object_name<'de, D>(deserializer: D) -> Result<&'static str, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(DeserializeMatches("related_card"))
}
