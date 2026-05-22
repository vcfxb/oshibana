use std::sync::LazyLock;
use enumflags2::bitflags;
use polars::chunked_array::builder::CategoricalChunkedBuilder;
use polars::datatypes::{CategoricalChunked, DataType};
use polars::error::PolarsResult;
use polars::prelude::{Categorical8Type, PlSmallStr};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;
use crate::enum_to_dt_enum;
use crate::traits::builder::PolarsBuilder;
use crate::traits::map_type::MapPolarsType;

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

pub static COLOR_DT: LazyLock<DataType> = LazyLock::new(|| enum_to_dt_enum::<Color>());

impl MapPolarsType for Color {
    type StaticPolarsType = Categorical8Type;
    type Builder = CategoricalChunkedBuilder<Categorical8Type>;

    fn dt() -> DataType {
        COLOR_DT.clone()
    }
}

impl PolarsBuilder<Color> for CategoricalChunkedBuilder<Categorical8Type> {
    type ChunkedType = CategoricalChunked<Categorical8Type>;

    fn new() -> Self {
        CategoricalChunkedBuilder::new(PlSmallStr::EMPTY, COLOR_DT.clone())
    }

    fn append(&mut self, val: Color) -> PolarsResult<()> {
        self.append_str(val.into())
    }

    fn append_null(&mut self) {
        self.append_null()
    }

    fn finish(self) -> PolarsResult<Self::ChunkedType> {
        Ok(self.finish())
    }
}
