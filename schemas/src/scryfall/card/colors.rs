use enum_ordinalize::Ordinalize;
use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;

#[bitflags]
#[derive(
    Copy, Clone, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd, EnumIter, IntoStaticStr,
    Serialize, Ordinalize, TypeName
)]
#[repr(u8)]
pub enum Color {
    W,
    U,
    B,
    R,
    G,
}
