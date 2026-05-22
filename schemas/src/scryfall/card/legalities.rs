use std::sync::LazyLock;
use polars::chunked_array::builder::CategoricalChunkedBuilder;
use polars::datatypes::{Categorical8Type, DataType};
use polars::error::PolarsResult;
use polars::prelude::{CategoricalChunked, PlSmallStr};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;
use crate::{enum_to_dt_enum, generate_record_builder_and_dt};
use crate::traits::builder::PolarsBuilder;
use crate::traits::map_type::MapPolarsType;

#[derive(
    Deserialize,
    Debug,
    Copy,
    Clone,
    Hash,
    Eq,
    PartialEq,
    Serialize,
    TypeName,
    IntoStaticStr,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Legality {
    Legal,
    NotLegal,
    Restricted,
    Banned,
}

pub static LEGALITY_DT: LazyLock<DataType> = LazyLock::new(|| enum_to_dt_enum::<Legality>());

impl MapPolarsType for Legality {
    type StaticPolarsType = Categorical8Type;
    type Builder = CategoricalChunkedBuilder<Categorical8Type>;

    fn dt() -> DataType {
        LEGALITY_DT.clone()
    }
}

impl PolarsBuilder<Legality> for CategoricalChunkedBuilder<Categorical8Type> {
    type ChunkedType = CategoricalChunked<Categorical8Type>;

    fn new() -> Self {
        CategoricalChunkedBuilder::new(PlSmallStr::EMPTY, Legality::dt())
    }

    fn append(&mut self, val: Legality) -> PolarsResult<()> {
        self.append_str(val.into())
    }

    fn append_null(&mut self) {
        self.append_null()
    }

    fn finish(self) -> PolarsResult<Self::ChunkedType> {
        Ok(self.finish())
    }
}


generate_record_builder_and_dt! {
    #[derive(Serialize, Deserialize, Copy, Clone, Debug)]
    Legalities {
        standard: Legality,
        historic: Legality,
        timeless: Legality,
        pioneer: Legality,
        modern: Legality,
        legacy: Legality,
        pauper: Legality,
        vintage: Legality,
        commander: Legality,
        oathbreaker: Legality,
        standardbrawl: Legality,
        brawl: Legality,
        alchemy: Legality,
        paupercommander: Legality,
        duel: Legality,
        oldschool: Legality,
        premodern: Legality,
    }
}
