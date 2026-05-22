use std::sync::LazyLock;
use polars::chunked_array::builder::CategoricalChunkedBuilder;
use polars::datatypes::{Categorical8Type, DataType, PlSmallStr};
use polars::error::PolarsResult;
use polars::prelude::CategoricalChunked;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;
use crate::enum_to_dt_enum;
use crate::traits::builder::PolarsBuilder;
use crate::traits::map_type::MapPolarsType;

#[derive(
    Deserialize,
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    TypeName,
    IntoStaticStr,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Layout {
    Normal,
    Split,
    Flip,
    Transform,
    ModalDfc,
    Meld,
    Leveler,
    Class,
    Case,
    Saga,
    Adventure,
    Prepare,
    Mutate,
    Prototype,
    Battle,
    Planar,
    Scheme,
    Vanguard,
    Token,
    DoubleFacedToken,
    Emblem,
    Augment,
    Host,
    ArtSeries,
    ReversibleCard,
}

pub static LAYOUT_DT: LazyLock<DataType> = LazyLock::new(|| enum_to_dt_enum::<Layout>());

impl MapPolarsType for Layout {
    type StaticPolarsType = Categorical8Type;
    type Builder = CategoricalChunkedBuilder<Categorical8Type>;

    fn dt() -> DataType {
        LAYOUT_DT.clone()
    }
}

impl PolarsBuilder<Layout> for CategoricalChunkedBuilder<Categorical8Type> {
    type ChunkedType = CategoricalChunked<Categorical8Type>;

    fn new() -> Self {
        CategoricalChunkedBuilder::new(PlSmallStr::EMPTY, LAYOUT_DT.clone())
    }

    fn append(&mut self, val: Layout) -> PolarsResult<()> {
        self.append_str(val.into())
    }

    fn append_null(&mut self) {
        self.append_null()
    }

    fn finish(self) -> PolarsResult<Self::ChunkedType> {
        Ok(self.finish())
    }
}
