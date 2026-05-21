use std::sync::LazyLock;
use polars::chunked_array::builder::CategoricalChunkedBuilder;
use polars::datatypes::{Categorical8Type, DataType, PlSmallStr};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;
use crate::enum_to_dt_enum;
use crate::traits::ChunkedArrayBuilder;

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

impl ChunkedArrayBuilder for Layout {
    type Builder = CategoricalChunkedBuilder<Categorical8Type>;

    fn dt() -> DataType {
        LAYOUT_DT.clone()
    }


    fn new_builder() -> Self::Builder {
        CategoricalChunkedBuilder::new(PlSmallStr::EMPTY, LAYOUT_DT.clone())
    }

    fn append(builder: &mut Self::Builder, val: Self) {
        builder.append_str(val.into()).unwrap()
    }
}
