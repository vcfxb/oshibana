use chrono::{DateTime, Utc};
use serde::Deserialize;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct BulkData {
    // this breaks DeserializeOwned, so don't bother for now
    // #[serde(deserialize_with = "deserialize_object_name")]
    // pub object: &'static str,
    pub has_more: bool,
    pub data: Vec<BulkDataItem>,
}

// fn deserialize_object_name<'de, D>(deserializer: D) -> Result<&'static str, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     deserializer.deserialize_str(DeserializeMatches("bulk_data"))
// }

#[derive(Deserialize, Debug)]
pub struct BulkDataItem {
    pub id: Uuid,
    pub uri: Url,
    pub r#type: String,
    pub name: String,
    pub description: String,
    pub download_uri: Url,
    pub updated_at: DateTime<Utc>,
    pub size: u64,
    pub content_type: String,
    pub content_encoding: String,
}
