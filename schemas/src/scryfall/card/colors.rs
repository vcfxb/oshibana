use crate::generate_enum_dt_map_and_builder_impl;
use enumflags2::bitflags;
use polars::prelude::Categorical8Type;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;

#[bitflags]
#[derive(
    Copy,
    Clone,
    Debug,
    Deserialize,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    EnumIter,
    IntoStaticStr,
    Serialize,
    TypeName,
)]
#[repr(u8)]
pub enum Color {
    W,
    U,
    B,
    R,
    G,
}

generate_enum_dt_map_and_builder_impl!(Color => Categorical8Type);
