use std::sync::LazyLock;
use enumflags2::bitflags;
use polars::chunked_array::builder::CategoricalChunkedBuilder;
use polars::chunked_array::ChunkedArray;
use polars::error::PolarsResult;
use polars::prelude::{Categorical8Type, CategoricalChunked, DataType, PlSmallStr};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;
use crate::enum_to_dt_enum;
use crate::traits::builder::PolarsBuilder;
use crate::traits::map_type::MapPolarsType;

#[bitflags]
#[derive(
    Deserialize,
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    TypeName,
    IntoStaticStr,
    EnumIter,
)]
#[repr(u32)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Language {
    /// English
    En,
    /// Spanish
    Es,
    /// French
    Fr,
    /// German
    De,
    /// Italian
    It,
    /// Portuguese
    Pt,
    /// Japanese
    Ja,
    /// Korean
    Ko,
    /// Russian
    Ru,
    /// Simplified Chinese
    Zhs,
    /// Traditional Chinese
    Zht,
    /// Hebrew
    He,
    /// Latin
    La,
    /// Ancient Greek
    Grc,
    /// Arabic
    Ar,
    /// Sanskrit
    Sa,
    /// Phyrexian
    Ph,
    /// Quenya -- lotr language
    Qya,
}

pub static LANGUAGE_DT: LazyLock<DataType> = LazyLock::new(|| enum_to_dt_enum::<Language>());

impl MapPolarsType for Language {
    type StaticPolarsType = Categorical8Type;
    type Builder = CategoricalChunkedBuilder<Categorical8Type>;

    fn dt() -> DataType {
        LANGUAGE_DT.clone()
    }
}

impl PolarsBuilder<Language> for CategoricalChunkedBuilder<Categorical8Type> {
    type ChunkedType = CategoricalChunked<Categorical8Type>;
    
    fn new() -> Self {
        CategoricalChunkedBuilder::new(PlSmallStr::EMPTY, LANGUAGE_DT.clone())
    }

    fn append(&mut self, val: Language) -> PolarsResult<()> {
        self.append_str(val.into())
    }

    fn append_null(&mut self) {
        self.append_null()
    }

    fn finish(self) -> PolarsResult<CategoricalChunked<Categorical8Type>> {
        Ok(self.finish())
    }
}
