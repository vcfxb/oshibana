use serde::Deserialize;

#[derive(Deserialize, Debug, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ImageStatus {
    Missing,
    Placeholder,
    Lowres,
    HighresScan,
}
