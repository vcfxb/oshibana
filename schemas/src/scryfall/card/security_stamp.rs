use polars::prelude::Categorical8Type;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;
use crate::generate_enum_dt_map_and_builder_impl;

#[derive(
    Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TypeName, IntoStaticStr, EnumIter,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum SecurityStamp {
    Oval,
    Triangle,
    Acorn,
    Circle,
    Arena,
    Heart,
}

generate_enum_dt_map_and_builder_impl!(SecurityStamp => Categorical8Type);
