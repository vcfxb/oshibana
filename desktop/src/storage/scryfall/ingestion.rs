//! Tools for efficiently ingesting scryfall data.

use std::any::Any;
use std::collections::HashMap;
use polars::chunked_array::builder::CategoricalChunkedBuilder;
use polars::prelude::{ListPrimitiveChunkedBuilder, PolarsCategoricalType, PolarsNumericType, PrimitiveChunkedBuilder};
use schemas::oshibana::scryfall::SCRYFALL_SCHEMA;

pub trait AnyChunkedBuilder {
}

impl<P: PolarsNumericType> AnyChunkedBuilder for PrimitiveChunkedBuilder<P> {

}

impl<C: PolarsCategoricalType> AnyChunkedBuilder for CategoricalChunkedBuilder<C> {

}

impl<P: PolarsNumericType> AnyChunkedBuilder for ListPrimitiveChunkedBuilder<P> {

}

pub fn make_builder_table() -> HashMap<&'static str, Box<dyn AnyChunkedBuilder>> {
    for (name, dtype) in SCRYFALL_SCHEMA.iter() {

    }

    todo!()
}

