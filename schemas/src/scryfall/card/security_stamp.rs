use enum_ordinalize::Ordinalize;
use serde::{Deserialize, Serialize};
use typename::TypeName;

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Ordinalize, TypeName)]
#[serde(rename_all = "lowercase")]
pub enum SecurityStamp {
    Oval,
    Triangle,
    Acorn,
    Circle,
    Arena,
    Heart,
}
