use crate::utils::array_to_bitflags::ExpectStr;
use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone, EnumIter, IntoStaticStr)]
#[serde(rename_all = "lowercase")]
#[bitflags]
#[repr(u32)]
pub enum FrameEffect {
    Legendary,
    Miracle,
    Enchantment,
    Draft,
    Devoid,
    Tombstone,
    Colorshifted,
    Inverted,
    SunMoonDfc,
    CompassLandDfc,
    OriginPwDfc,
    MoonEldraziDfc,
    WaxingAndWaningMoonDfc,
    Showcase,
    ExtendedArt,
    Companion,
    Etched,
    Snow,
    Lesson,
    ShatteredGlass,
    ConvertDfc,
    FanDfc,
    UpsideDownDfc,
    Spree,
}

impl ExpectStr for FrameEffect {
    const EXPECT_STR: &'static str = "frame effect";
}
