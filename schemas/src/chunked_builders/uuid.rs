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
I wish I cou
 */


// impl MapPolarsType for Option<Uuid> {
//     type StaticPolarsType = <Uuid as MapPolarsType>::StaticPolarsType;
//     type Builder = <Uuid as MapPolarsType>::Builder;
//
//     fn dt() -> DataType {
//         <Uuid as MapPolarsType>::dt()
//     }
// }

// impl PolarsBuilder<Uuid> for PrimitiveChunkedBuilder<UInt128Type> {
//     type ChunkedType = ChunkedArray<UInt128Type>;
//
//     fn new() -> Self {
//         PrimitiveChunkedBuilder::new(PlSmallStr::EMPTY, 0)
//     }
//
//     fn append(&mut self, val: Uuid) -> PolarsResult<()> {
//         ChunkedBuilder::append_value(self, val.as_u128());
//         Ok(())
//     }
//
//     fn append_null(&mut self) {
//         ChunkedBuilder::append_null(self)
//     }
//
//     fn finish(self) -> PolarsResult<ChunkedArray<UInt128Type>> {
//         Ok(ChunkedBuilder::finish(self))
//     }
// }

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