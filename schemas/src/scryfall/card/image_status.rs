use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Eq, PartialEq, Copy, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageStatus {
    Missing,
    Placeholder,
    Lowres,
    HighresScan,
}
