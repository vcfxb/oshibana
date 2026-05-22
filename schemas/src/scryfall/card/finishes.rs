use std::sync::LazyLock;
use enumflags2::bitflags;
use polars::chunked_array::builder::CategoricalChunkedBuilder;
use polars::datatypes::{Categorical8Type, DataType};
use polars::error::PolarsResult;
use polars::prelude::{CategoricalChunked, PlSmallStr};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;
use crate::enum_to_dt_enum;
use crate::traits::builder::PolarsBuilder;
use crate::traits::map_type::MapPolarsType;

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

pub static FINISH_DT: LazyLock<DataType> = LazyLock::new(|| enum_to_dt_enum::<Finish>());

impl MapPolarsType for Finish {
    type StaticPolarsType = Categorical8Type;
    type Builder = CategoricalChunkedBuilder<Categorical8Type>;

    fn dt() -> DataType {
        FINISH_DT.clone()
    }
}

impl PolarsBuilder<Finish> for CategoricalChunkedBuilder<Categorical8Type> {
    type ChunkedType = CategoricalChunked<Categorical8Type>;

    fn new() -> Self {
        CategoricalChunkedBuilder::new(PlSmallStr::EMPTY, FINISH_DT.clone())
    }

    fn append(&mut self, val: Finish) -> PolarsResult<()> {
        self.append_str(val.into())
    }

    fn append_null(&mut self) {
        self.append_null()
    }

    fn finish(self) -> PolarsResult<Self::ChunkedType> {
        Ok(self.finish())
    }
}
