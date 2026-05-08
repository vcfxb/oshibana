use enum_ordinalize::Ordinalize;
use serde::{Deserialize, Serialize};
use typename::TypeName;

#[derive(Deserialize, Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Ordinalize, TypeName)]
#[serde(rename_all = "snake_case")]
pub enum Legality {
    Legal,
    NotLegal,
    Restricted,
    Banned,
}
