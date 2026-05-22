//! Trait to map rust types to polars types

mod primitive;
mod string;

use polars::chunked_array::builder::AnonymousOwnedListBuilder;
use polars::datatypes::{DataType, PolarsDataType};
use polars::prelude::ListType;
use crate::traits::builder::PolarsBuilder;

pub trait MapPolarsType: Sized {
    type StaticPolarsType: PolarsDataType;
    type Builder: PolarsBuilder<Self>;

    fn dt() -> DataType;
}

impl<T> MapPolarsType for Option<T>
where
    T: MapPolarsType,
    <T as MapPolarsType>::Builder: PolarsBuilder<Option<T>>
{
    type StaticPolarsType = T::StaticPolarsType;
    type Builder = T::Builder;

    fn dt() -> DataType {
        T::dt()
    }
}

impl<T: MapPolarsType> MapPolarsType for Vec<T> {
    type StaticPolarsType = ListType;
    type Builder = AnonymousOwnedListBuilder;

    fn dt() -> DataType {
        DataType::List(Box::new(T::dt()))
    }
}
