use crate::traits::builder::PolarsBuilder;
use crate::traits::map_type::MapPolarsType;
use polars::chunked_array::ChunkedArray;
use polars::datatypes::DataType;
use polars::error::PolarsResult;
use polars::prelude::{PlSmallStr, StringChunkedBuilder, StringType};
use uuid::Uuid;

impl MapPolarsType for Uuid {
    type StaticPolarsType = StringType;
    type Builder = StringChunkedBuilder;

    fn dt() -> DataType {
        DataType::String
    }
}

/*
I wish I could use u128s (or better yet, a native uuid type)
here but polars support for them is not finished it seems.
https://github.com/pola-rs/polars/pull/27881
 */

impl PolarsBuilder<Uuid> for StringChunkedBuilder {
    type ChunkedType = ChunkedArray<StringType>;

    fn new() -> Self {
        StringChunkedBuilder::new(PlSmallStr::EMPTY, 0)
    }

    fn append(&mut self, val: Uuid) -> PolarsResult<()> {
        self.append_value(val.to_string());
        Ok(())
    }

    fn append_null(&mut self) {
        self.append_option::<String>(None);
    }

    fn finish(self) -> PolarsResult<Self::ChunkedType> {
        Ok(self.finish())
    }
}

impl PolarsBuilder<Option<Uuid>> for StringChunkedBuilder {
    type ChunkedType = ChunkedArray<StringType>;

    fn new() -> Self {
        <Self as PolarsBuilder<Uuid>>::new()
    }

    fn append(&mut self, val: Option<Uuid>) -> PolarsResult<()> {
        self.append_option(val.map(|uuid| uuid.to_string()));
        Ok(())
    }

    fn append_null(&mut self) {
        self.append_option::<String>(None);
    }

    fn finish(self) -> PolarsResult<Self::ChunkedType> {
        Ok(self.finish())
    }
}
