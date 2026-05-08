use crate::utils::deserialize_matches::DeserializeMatches;
use serde::Deserialize;
use serde::Deserializer;
use url::Url;

#[derive(Deserialize, Debug)]
pub struct ScryfallList<T> {
    // #[serde(deserialize_with = "deserialize_object_name")]
    // pub object: &'static str,
    pub data: Vec<T>,
    pub next_page: Option<Url>,
    pub total_cards: Option<u64>,
    pub warnings: Vec<String>,
}

// fn deserialize_object_name<'de, D>(deserializer: D) -> Result<&'static str, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     deserializer.deserialize_str(DeserializeMatches("list"))
// }
