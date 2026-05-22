use enumflags2::bitflags;
use polars::datatypes::Categorical8Type;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;
use crate::generate_enum_dt_map_and_builder_impl;

#[bitflags]
#[derive(
    Deserialize,
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    TypeName,
    EnumIter,
    IntoStaticStr,
)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum Finish {
    Foil,
    Nonfoil,
    Etched,
}

generate_enum_dt_map_and_builder_impl!(Finish => Categorical8Type);
