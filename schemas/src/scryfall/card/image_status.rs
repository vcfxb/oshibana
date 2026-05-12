use enum_ordinalize::Ordinalize;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;

#[derive(
    Deserialize, 
    Debug, 
    Eq, PartialEq,
    Copy, Clone, 
    Serialize, 
    Ordinalize, 
    TypeName,
    EnumIter, IntoStaticStr
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ImageStatus {
    Missing,
    Placeholder,
    Lowres,
    HighresScan,
}
