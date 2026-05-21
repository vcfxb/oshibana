//! Schemas for our various input and output formats

use std::sync::Arc;
use polars::datatypes::{DataType, FrozenCategories};
use strum::IntoEnumIterator;

pub mod oshibana;
pub mod scryfall;
pub mod macros;
pub mod traits;
pub mod chunked_builders;

/// Utility function to generate a [polars] categorical datatype for an enum.
fn enum_to_dt_enum<T: IntoEnumIterator + Into<&'static str>>() -> DataType {
    let str_iter = T::iter().map(|variant| variant.into());
    let cats = FrozenCategories::new(str_iter).unwrap();
    let mapping = Arc::clone(cats.mapping());
    DataType::Enum(cats, mapping)
}
