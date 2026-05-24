use crate::generate_enum_dt_map_and_builder_impl;
use polars::datatypes::Categorical8Type;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};


/// Frame effects as described at https://scryfall.com/docs/api/frames.
///
/// There are a few undocumented ones here, which I found in live data on the Scryfall API.
/// I suspect those originated from MTGJSON
/// https://mtgjson.com/data-models/card/card-set/#frameeffects
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone, EnumIter, IntoStaticStr)]
#[serde(rename_all = "lowercase")]
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

    /// Not documented anywhere afaik -- perhaps a bug that some can have this
    FullArt,

    /// Another undocumented variant. Introduced for a deadpool secret lair it seems.
    /// https://scryfall.com/card/sld/7128/mountain?utm_source=api
    DazzleFoil,

    /// Also undocumented, textless cards from FRA get their own variant it seems.
    /// https://scryfall.com/card/fra/402/bloodline-recollector-ancestral-craving?utm_source=api
    Textless
}

const _: () = const {
    assert!(
        size_of::<FrameEffect>() == size_of::<u8>(),
        "frame effect needs larger categorical type"
    );
};

generate_enum_dt_map_and_builder_impl!(FrameEffect => Categorical8Type);
