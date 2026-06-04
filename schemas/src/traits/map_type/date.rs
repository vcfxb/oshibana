use crate::traits::builder::PolarsBuilder;
use crate::traits::map_type::MapPolarsType;
use chrono::{DateTime, Datelike, NaiveDate};
use polars::datatypes::DataType;
use polars::error::PolarsResult;
use polars::prelude::{
    ChunkedBuilder, DateChunked, DateType, Int32Type, PlSmallStr, PrimitiveChunkedBuilder,
};

impl MapPolarsType for NaiveDate {
    type StaticPolarsType = DateType;
    type Builder = PrimitiveChunkedBuilder<Int32Type>;

    fn dt() -> DataType {
        DataType::Date
    }
}

impl PolarsBuilder<NaiveDate> for PrimitiveChunkedBuilder<Int32Type> {
    type ChunkedType = DateChunked;

    fn new() -> Self {
        PrimitiveChunkedBuilder::new(PlSmallStr::EMPTY, 0)
    }

    fn append(&mut self, val: NaiveDate) -> PolarsResult<()> {
        // very stupid workaround for polars people not releasing since they fixed
        // https://github.com/pola-rs/polars/issues/26577
        self.append_value(
            val.num_days_from_ce() - DateTime::UNIX_EPOCH.date_naive().num_days_from_ce(),
        );
        Ok(())
    }

    fn append_null(&mut self) {
        ChunkedBuilder::append_null(self)
    }

    fn finish(self) -> PolarsResult<Self::ChunkedType> {
        Ok(ChunkedBuilder::finish(self).into_date())
    }
}
