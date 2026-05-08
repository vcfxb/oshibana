use enum_ordinalize::Ordinalize;
use serde::{Deserialize, Serialize};
use typename::TypeName;

#[derive(Deserialize, Debug, Eq, PartialEq, Copy, Clone, Serialize, Ordinalize, TypeName)]
#[serde(rename_all = "snake_case")]
pub enum ImageStatus {
    Missing,
    Placeholder,
    Lowres,
    HighresScan,
}
