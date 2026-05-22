use std::sync::LazyLock;
use polars::chunked_array::builder::CategoricalChunkedBuilder;
use polars::error::PolarsResult;
use polars::prelude::{Categorical8Type, CategoricalChunked, DataType, PlSmallStr};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;
use url::Url;
use uuid::Uuid;
use crate::{enum_to_dt_enum, generate_record_builder_and_dt};
use crate::traits::builder::PolarsBuilder;
use crate::traits::map_type::MapPolarsType;

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
#[strum(serialize_all = "snake_case")]
pub enum Component {
    Token,
    MeldPart,
    MeldResult,
    ComboPiece,
}

pub static COMPONENT_DT: LazyLock<DataType> = LazyLock::new(|| enum_to_dt_enum::<Component>());

impl MapPolarsType for Component {
    type StaticPolarsType = Categorical8Type;
    type Builder = CategoricalChunkedBuilder<Categorical8Type>;

    fn dt() -> DataType {
        COMPONENT_DT.clone()
    }
}

impl PolarsBuilder<Component> for CategoricalChunkedBuilder<Categorical8Type> {
    type ChunkedType = CategoricalChunked<Categorical8Type>;

    fn new() -> Self {
        CategoricalChunkedBuilder::new(PlSmallStr::EMPTY, COMPONENT_DT.clone())
    }

    fn append(&mut self, val: Component) -> PolarsResult<()> {
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
    #[derive(Deserialize, Debug, Serialize)]
    RelatedCard {
        id: Uuid,
        component: Component,
        name: String,
        type_line: String,
        uri: Url,
    }
}
