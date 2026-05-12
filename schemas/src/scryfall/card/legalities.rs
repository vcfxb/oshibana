use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;

#[derive(
    Deserialize,
    Debug,
    Copy,
    Clone,
    Hash,
    Eq,
    PartialEq,
    Serialize,
    TypeName,
    IntoStaticStr,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Legality {
    Legal,
    NotLegal,
    Restricted,
    Banned,
}
