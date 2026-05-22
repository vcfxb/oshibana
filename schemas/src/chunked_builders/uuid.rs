use crate::traits::builder::PolarsBuilder;
use crate::traits::map_type::MapPolarsType;
use polars::chunked_array::ChunkedArray;
use polars::datatypes::DataType;
use polars::error::PolarsResult;
use polars::prelude::{ChunkedBuilder, PlSmallStr, PrimitiveChunkedBuilder, UInt128Type};
use uuid::Uuid;

impl MapPolarsType for Uuid {
    type StaticPolarsType = UInt128Type;
    type Builder = PrimitiveChunkedBuilder<UInt128Type>;

    fn dt() -> DataType {
        DataType::UInt128
    }
}

// impl MapPolarsType for Option<Uuid> {
//     type StaticPolarsType = <Uuid as MapPolarsType>::StaticPolarsType;
//     type Builder = <Uuid as MapPolarsType>::Builder;
//
//     fn dt() -> DataType {
//         <Uuid as MapPolarsType>::dt()
//     }
// }

impl PolarsBuilder<Uuid> for PrimitiveChunkedBuilder<UInt128Type> {
    type ChunkedType = ChunkedArray<UInt128Type>;

    fn new() -> Self {
        PrimitiveChunkedBuilder::new(PlSmallStr::EMPTY, 0)
    }

    fn append(&mut self, val: Uuid) -> PolarsResult<()> {
        ChunkedBuilder::append_value(self, val.as_u128());
        Ok(())
    }

    fn append_null(&mut self) {
        ChunkedBuilder::append_null(self)
    }

    fn finish(self) -> PolarsResult<ChunkedArray<UInt128Type>> {
        Ok(ChunkedBuilder::finish(self))
    }
}
