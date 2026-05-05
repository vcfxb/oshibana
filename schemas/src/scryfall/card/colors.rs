use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};

#[bitflags]
#[derive(
    Copy, Clone, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd, EnumIter, IntoStaticStr,
    Serialize
)]
#[repr(u8)]
pub enum Color {
    W,
    U,
    B,
    R,
    G,
}
