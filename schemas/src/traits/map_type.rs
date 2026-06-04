//! Trait to map rust types to polars types

mod boolean;
mod date;
mod primitive;
mod string;
mod time;

use crate::traits::builder::PolarsBuilder;
use polars::chunked_array::builder::AnonymousOwnedListBuilder;
use polars::datatypes::{DataType, PolarsDataType};
use polars::prelude::ListType;

pub trait MapPolarsType: Sized {
    type StaticPolarsType: PolarsDataType;
    type Builder: PolarsBuilder<Self>;

    fn dt() -> DataType;
}

impl<T> MapPolarsType for Option<T>
where
    T: MapPolarsType,
    <T as MapPolarsType>::Builder: PolarsBuilder<Option<T>>,
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
