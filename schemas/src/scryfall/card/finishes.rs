use enum_ordinalize::Ordinalize;
use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use typename::TypeName;

#[bitflags]
#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Ordinalize, TypeName)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum Finish {
    Foil,
    Nonfoil,
    Etched,
}
