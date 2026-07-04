use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Debug)]
pub struct ScryfallList<T> {
    pub data: Vec<T>,
    pub next_page: Option<Url>,
    pub total_cards: Option<u64>,
    #[serde(default)]
    pub warnings: Vec<String>,
}
