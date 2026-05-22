use polars::datatypes::DataType;
use polars::error::PolarsResult;
use polars::prelude::{PlSmallStr, StringChunked, StringChunkedBuilder, StringType};
use url::Url;
use crate::traits::builder::PolarsBuilder;
use crate::traits::map_type::MapPolarsType;

impl MapPolarsType for Url {
    type StaticPolarsType = StringType;
    type Builder = StringChunkedBuilder;

    fn dt() -> DataType {
        DataType::String
    }
}

impl PolarsBuilder<Url> for StringChunkedBuilder {
    type ChunkedType = StringChunked;

    fn new() -> Self {
        StringChunkedBuilder::new(PlSmallStr::EMPTY, 0)
    }

    fn append(&mut self, val: Url) -> PolarsResult<()> {
        StringChunkedBuilder::append_value(self, val);
        Ok(())
    }

    fn append_null(&mut self) {
        self.append_null()
    }

    fn finish(self) -> PolarsResult<Self::ChunkedType> {
        Ok(StringChunkedBuilder::finish(self))
    }
}


impl PolarsBuilder<Option<Url>> for StringChunkedBuilder {
    type ChunkedType = StringChunked;

    fn new() -> Self {
        <Self as PolarsBuilder<Url>>::new()
    }

    fn append(&mut self, val: Option<Url>) -> PolarsResult<()> {
        match val {
            Some(s) => <Self as PolarsBuilder<Url>>::append(self, s),
            None => {
                self.append_null();
                Ok(())
            }
        }
    }

    fn append_null(&mut self) {
        StringChunkedBuilder::append_null(self)
    }

    fn finish(self) -> PolarsResult<Self::ChunkedType> {
        <Self as PolarsBuilder<Url>>::finish(self)
    }
}
