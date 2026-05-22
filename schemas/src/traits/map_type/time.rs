use crate::traits::builder::PolarsBuilder;
use crate::traits::map_type::MapPolarsType;
use chrono::{DateTime, Utc};
use polars::datatypes::DataType;
use polars::error::PolarsResult;
use polars::prelude::{
    ChunkedBuilder, DatetimeChunked, DatetimeType, Int64Type, PlSmallStr, PrimitiveChunkedBuilder,
    TimeUnit, TimeZone,
};

impl MapPolarsType for DateTime<Utc> {
    type StaticPolarsType = DatetimeType;
    type Builder = PrimitiveChunkedBuilder<Int64Type>;

    fn dt() -> DataType {
        DataType::Datetime(TimeUnit::Milliseconds, Some(TimeZone::UTC))
    }
}

impl PolarsBuilder<DateTime<Utc>> for PrimitiveChunkedBuilder<Int64Type> {
    type ChunkedType = DatetimeChunked;

    fn new() -> Self {
        PrimitiveChunkedBuilder::new(PlSmallStr::EMPTY, 0)
    }

    fn append(&mut self, val: DateTime<Utc>) -> PolarsResult<()> {
        self.append_value(val.timestamp_millis());
        Ok(())
    }

    fn append_null(&mut self) {
        ChunkedBuilder::append_null(self)
    }

    fn finish(self) -> PolarsResult<Self::ChunkedType> {
        let prim_ca = ChunkedBuilder::finish(self);
        Ok(prim_ca.into_datetime(TimeUnit::Milliseconds, Some(TimeZone::UTC)))
    }
}
