use polars::prelude::Categorical8Type;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;
use crate::generate_enum_dt_map_and_builder_impl;

#[derive(
    Deserialize, Debug, Eq, PartialEq, Copy, Clone, Serialize, TypeName, EnumIter, IntoStaticStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ImageStatus {
    Missing,
    Placeholder,
    Lowres,
    HighresScan,
}

generate_enum_dt_map_and_builder_impl!(ImageStatus => Categorical8Type);