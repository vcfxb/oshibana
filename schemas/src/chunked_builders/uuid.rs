use polars::chunked_array::ChunkedArray;
use polars::datatypes::DataType;
use polars::prelude::{ChunkedBuilder, PlSmallStr, PrimitiveChunkedBuilder, UInt128Type};
use uuid::Uuid;
use crate::traits::ChunkedArrayBuilder;

pub struct UuidChunkedBuilder(PrimitiveChunkedBuilder<UInt128Type>);

impl ChunkedBuilder<Uuid, UInt128Type> for UuidChunkedBuilder {
    fn append_value(&mut self, val: Uuid) {
        self.0.append_value(val.as_u128())
    }

    fn append_null(&mut self) {
        self.0.append_null()
    }

    fn finish(self) -> ChunkedArray<UInt128Type> {
        self.0.finish()
    }

    fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit()
    }
}

impl ChunkedArrayBuilder for Uuid {
    type Builder = UuidChunkedBuilder;

    fn dt() -> DataType {
        DataType::UInt128
    }


    fn new_builder() -> Self::Builder {
        UuidChunkedBuilder(PrimitiveChunkedBuilder::new(PlSmallStr::EMPTY, 0))
    }

    fn append(builder: &mut Self::Builder, val: Self) {
        builder.append_value(val)
    }
}

impl ChunkedArrayBuilder for Option<Uuid> {
    type Builder = <Uuid as ChunkedArrayBuilder>::Builder;

    fn dt() -> DataType {
        DataType::UInt128
    }


    fn new_builder() -> Self::Builder {
        <Uuid as ChunkedArrayBuilder>::new_builder()
    }

    fn append(builder: &mut Self::Builder, val: Self) {
        builder.append_option(val)
    }
}
