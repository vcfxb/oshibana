use std::sync::LazyLock;
use enumflags2::bitflags;
use polars::chunked_array::builder::CategoricalChunkedBuilder;
use polars::prelude::{Categorical8Type, DataType, PlSmallStr};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;
use crate::enum_to_dt_enum;
use crate::traits::ChunkedArrayBuilder;

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

impl ChunkedArrayBuilder for Language {
    type Builder = CategoricalChunkedBuilder<Categorical8Type>;

    fn dt() -> DataType {
        LANGUAGE_DT.clone()
    }

    fn new_builder() -> Self::Builder {
        CategoricalChunkedBuilder::new(PlSmallStr::EMPTY, LANGUAGE_DT.clone())
    }

    fn append(builder: &mut Self::Builder, val: Self) {
        builder.append_str(val.into()).unwrap()
    }
}
