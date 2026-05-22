use polars::datatypes::DataType;
use polars::error::PolarsResult;
use polars::prelude::{BooleanChunked, BooleanChunkedBuilder, BooleanType, ChunkedBuilder, PlSmallStr};
use crate::traits::builder::PolarsBuilder;
use crate::traits::map_type::MapPolarsType;

impl MapPolarsType for bool {
    type StaticPolarsType = BooleanType;
    type Builder = BooleanChunkedBuilder;

    fn dt() -> DataType {
        DataType::Boolean
    }
}

impl PolarsBuilder<bool> for BooleanChunkedBuilder {
    type ChunkedType = BooleanChunked;

    fn new() -> Self {
        BooleanChunkedBuilder::new(PlSmallStr::EMPTY, 0)
    }

    fn append(&mut self, val: bool) -> PolarsResult<()> {
        Ok(self.append_value(val))
    }

    fn append_null(&mut self) {
        ChunkedBuilder::append_null(self)
    }

    fn finish(self) -> PolarsResult<Self::ChunkedType> {
        Ok(ChunkedBuilder::finish(self))
    }
}

impl PolarsBuilder<Option<bool>> for BooleanChunkedBuilder {
    type ChunkedType = <Self as PolarsBuilder<bool>>::ChunkedType;

    fn new() -> Self {
        <Self as PolarsBuilder<bool>>::new()
    }

    fn append(&mut self, val: Option<bool>) -> PolarsResult<()> {
        match val {
            Some(v) => <Self as PolarsBuilder<bool>>::append(self, v),
            None => Ok(<Self as PolarsBuilder<bool>>::append_null(self)),
        }
    }

    fn append_null(&mut self) {
        <Self as PolarsBuilder<bool>>::append_null(self)
    }

    fn finish(self) -> PolarsResult<Self::ChunkedType> {
        <Self as PolarsBuilder<bool>>::finish(self)
    }
}
