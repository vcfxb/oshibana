//! Oshibana schema, including oshibana compressions of scryfall data stored on disc.

use polars::datatypes::{DataType, Field};

// pub mod scryfall;
pub mod user_data;

// util functions for constructing polars schemas

fn field(name: &'static str, dt: DataType) -> Field {
    Field::new(name.into(), dt)
}

fn list(of: DataType) -> DataType {
    DataType::List(Box::new(of))
}
