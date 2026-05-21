//! Tools for efficiently ingesting scryfall data.

use std::collections::HashMap;
use polars::chunked_array::builder::CategoricalChunkedBuilder;
use polars::prelude::{ListPrimitiveChunkedBuilder, PolarsCategoricalType, PolarsNumericType, PrimitiveChunkedBuilder};
// use schemas::oshibana::scryfall::SCRYFALL_SCHEMA;
