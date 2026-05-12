//! Oshibana schema, including oshibana compressions of scryfall data stored on disc.

use std::sync::Arc;
use polars::datatypes::{DataType, Field, FrozenCategories};
use strum::IntoEnumIterator;

pub mod user_data;
pub mod scryfall;

// util functions for constructing polars schemas

fn enum_to_dt_enum<T: IntoEnumIterator + Into<&'static str>>() -> DataType {
    let str_iter = T::iter()
        .map(|variant| variant.into());

    let cats = FrozenCategories::new(str_iter).unwrap();
    let mapping = Arc::clone(cats.mapping());
    DataType::Enum(cats, mapping)
}

fn field(name: &'static str, dt: DataType) -> Field {
    Field::new(name.into(), dt)
}

fn list(of: DataType) -> DataType {
    DataType::List(Box::new(of))
}
