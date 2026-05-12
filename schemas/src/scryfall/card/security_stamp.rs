use enum_ordinalize::Ordinalize;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;

#[derive(
    Copy,
    Clone,
    Debug,
    Deserialize,
    Eq,
    PartialEq,
    Serialize,
    Ordinalize,
    TypeName,
    IntoStaticStr,
    EnumIter,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SecurityStamp {
    Oval,
    Triangle,
    Acorn,
    Circle,
    Arena,
    Heart,
}
