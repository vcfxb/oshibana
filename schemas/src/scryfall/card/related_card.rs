use std::borrow::Cow;
use std::sync::LazyLock;
use polars::chunked_array::builder::{AnonymousOwnedListBuilder, CategoricalChunkedBuilder};
use polars::error::PolarsResult;
use polars::prelude::{Categorical8Type, CategoricalChunked, DataType, IntoSeries, ListBuilderTrait, PlSmallStr};
use polars::series::Series;
use serde::{Deserialize, Deserializer, Serialize};
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
        // #[serde(borrow)]
        // pub name: Cow<'a, str>,

        name: String,

        // #[serde(borrow)]
        // pub type_line: Cow<'a, str>,

        type_line: String,

        uri: Url,
    }
}
//
// impl ChunkedArrayBuilder for RelatedCard {
//     type Builder = RelatedCardBuilder;
//
//     fn dt() -> DataType {
//         RELATED_CARD_STRUCT_DT.clone()
//     }
//
//     fn new_builder() -> Self::Builder {
//         RelatedCardBuilder::new()
//     }
//
//     fn append(builder: &mut Self::Builder, val: Self) {
//         builder.append(val)
//     }
// }
//
// impl ChunkedArrayBuilder for Vec<RelatedCard> {
//     type Builder = AnonymousOwnedListBuilder;
//
//     fn dt() -> DataType {
//         DataType::List(Box::new(RELATED_CARD_STRUCT_DT.clone()))
//     }
//
//     fn new_builder() -> Self::Builder {
//         AnonymousOwnedListBuilder::new(
//             PlSmallStr::EMPTY,
//             0,
//             Some(RELATED_CARD_STRUCT_DT.clone())
//         )
//     }
//
//     fn append(builder: &mut Self::Builder, val: Self) {
//         let mut rel_card_ca_builder = RelatedCardBuilder::new();
//
//         for card in val {
//             rel_card_ca_builder.append(card);
//         }
//
//         let series = rel_card_ca_builder.finish().unwrap().into_series();
//         builder.append_series(&series).unwrap();
//     }
// }
//
// impl ChunkedArrayBuilder for Option<Vec<RelatedCard>> {
//     type Builder = <Vec<RelatedCard> as ChunkedArrayBuilder>::Builder;
//
//     fn dt() -> DataType {
//         <Vec<RelatedCard> as ChunkedArrayBuilder>::dt()
//     }
//
//     fn new_builder() -> Self::Builder {
//         <Vec<RelatedCard> as ChunkedArrayBuilder>::new_builder()
//     }
//
//     fn append(builder: &mut Self::Builder, val: Self) {
//         if let Some(v) = val {
//             <Vec<RelatedCard> as ChunkedArrayBuilder>::append(builder, v)
//         } else {
//             builder.append_null()
//         }
//     }
// }
