use crate::generate_enum_dt_map_and_builder_impl;
use enumflags2::bitflags;
use polars::datatypes::Categorical8Type;
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

generate_enum_dt_map_and_builder_impl!( Game => Categorical8Type );
