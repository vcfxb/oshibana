use crate::utils::array_to_bitflags::ExpectStr;
use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
use strum::IntoStaticStr;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone, EnumIter, IntoStaticStr)]
#[serde(rename_all = "lowercase")]
#[bitflags]
#[repr(u32)]
pub enum Game {
    Paper,
    Arena,
    Mtgo,
    Astral,
    Sega,
}

impl ExpectStr for Game {
    const EXPECT_STR: &'static str = "game variant";
}
