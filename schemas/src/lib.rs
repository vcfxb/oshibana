//! Schemas for our various input and output formats

use polars::datatypes::{DataType, FrozenCategories};
use std::sync::Arc;
use strum::IntoEnumIterator;

// pub mod oshibana;
pub mod chunked_builders;
pub mod macros;
pub mod oshibana;
pub mod scryfall;
pub mod traits;

/// Utility function to generate a [polars] categorical datatype for an enum.
fn enum_to_dt_enum<T: IntoEnumIterator + Into<&'static str>>() -> DataType {
    let str_iter = T::iter().map(|variant| variant.into());
    let cats = FrozenCategories::new(str_iter).unwrap();
    let mapping = Arc::clone(cats.mapping());
    DataType::Enum(cats, mapping)
}
