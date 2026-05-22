use crate::generate_enum_dt_map_and_builder_impl;
use polars::prelude::Categorical8Type;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize, EnumIter, IntoStaticStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Special,
    Mythic,
    Bonus,
}

generate_enum_dt_map_and_builder_impl!(Rarity => Categorical8Type);
