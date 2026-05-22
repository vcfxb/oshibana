use crate::traits::builder::PolarsBuilder;
use crate::traits::map_type::MapPolarsType;
use polars::datatypes::DataType;
use polars::error::PolarsResult;
use polars::prelude::{PlSmallStr, StringChunked, StringChunkedBuilder, StringType};

impl MapPolarsType for String {
    type StaticPolarsType = StringType;
    type Builder = StringChunkedBuilder;

    fn dt() -> DataType {
        DataType::String
    }
}

impl PolarsBuilder<String> for StringChunkedBuilder {
    type ChunkedType = StringChunked;

    fn new() -> Self {
        StringChunkedBuilder::new(PlSmallStr::EMPTY, 0)
    }

    fn append(&mut self, val: String) -> PolarsResult<()> {
        self.append_value(val);
        Ok(())
    }

    fn append_null(&mut self) {
        self.append_null()
    }

    fn finish(self) -> PolarsResult<Self::ChunkedType> {
        Ok(self.finish())
    }
}

impl PolarsBuilder<Option<String>> for StringChunkedBuilder {
    type ChunkedType = StringChunked;

    fn new() -> Self {
        <Self as PolarsBuilder<String>>::new()
    }

    fn append(&mut self, val: Option<String>) -> PolarsResult<()> {
        match val {
            Some(s) => <Self as PolarsBuilder<String>>::append(self, s),
            None => {
                self.append_null();
                Ok(())
            }
        }
    }

    fn append_null(&mut self) {
        self.append_null()
    }

    fn finish(self) -> PolarsResult<Self::ChunkedType> {
        <Self as PolarsBuilder<String>>::finish(self)
    }
}
