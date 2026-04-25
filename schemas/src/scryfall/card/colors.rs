use crate::utils::array_to_bitflags::ExpectStr;
use enumflags2::bitflags;
use serde::Deserialize;
use strum::{EnumIter, IntoStaticStr};

#[bitflags]
#[derive(
    Copy, Clone, Debug, Deserialize, Eq, PartialEq, Ord, PartialOrd, EnumIter, IntoStaticStr,
)]
#[repr(u8)]
pub enum Color {
    W,
    U,
    B,
    R,
    G,
}

impl ExpectStr for Color {
    const EXPECT_STR: &'static str = "color";
}
