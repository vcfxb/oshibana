use polars::chunked_array::ChunkedArray;
use polars::datatypes::DataType;
use polars::prelude::{ChunkedBuilder, PlSmallStr, StringChunkedBuilder, StringType};
use url::Url;
use crate::traits::ChunkedArrayBuilder;

pub type UrlChunkedBuilder = StringChunkedBuilder;

impl ChunkedArrayBuilder for Url {
    type Builder = UrlChunkedBuilder;

    fn dt() -> DataType {
        DataType::String
    }


    fn new_builder() -> Self::Builder {
        UrlChunkedBuilder::new(PlSmallStr::EMPTY, 0)
    }

    fn append(builder: &mut Self::Builder, val: Self) {
        builder.append_value(val)
    }
}

impl ChunkedArrayBuilder for Option<Url> {
    type Builder = <Url as ChunkedArrayBuilder>::Builder;

    fn dt() -> DataType {
        DataType::String
    }


    fn new_builder() -> Self::Builder {
        <Url as ChunkedArrayBuilder>::new_builder()
    }

    fn append(builder: &mut Self::Builder, val: Self) {
        builder.append_option(val)
    }
}
