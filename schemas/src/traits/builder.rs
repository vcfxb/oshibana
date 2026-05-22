use crate::traits::map_type::MapPolarsType;
use polars::chunked_array::ChunkedArray;
use polars::chunked_array::builder::{AnonymousOwnedListBuilder, CategoricalChunkedBuilder};
use polars::datatypes::{PlSmallStr, PolarsNumericType};
use polars::error::PolarsResult;
use polars::prelude::{ChunkedBuilder, IntoSeries, ListChunked, PrimitiveChunkedBuilder};
use polars::prelude::{ListBuilderTrait, PolarsCategoricalType};

pub trait PolarsBuilder<T: MapPolarsType> {
    type ChunkedType: IntoSeries;

    fn new() -> Self;

    fn append(&mut self, val: T) -> PolarsResult<()>;

    fn append_null(&mut self);

    fn finish(self) -> PolarsResult<Self::ChunkedType>;
}

impl<T> PolarsBuilder<Option<T>> for PrimitiveChunkedBuilder<<T as MapPolarsType>::StaticPolarsType>
where
    // this might be more constrained than it needs to be, but I think it works for now
    T: MapPolarsType<Builder = Self>,
    <T as MapPolarsType>::StaticPolarsType: PolarsNumericType,
    // Do Not reintroduce the ChunkedBuilder bound, it breaks trait inference in strange and
    // confusing ways.
    Self: PolarsBuilder<T>, // + ChunkedBuilder<T, <T as MapPolarsType>::StaticPolarsType>
{
    type ChunkedType = ChunkedArray<<Option<T> as MapPolarsType>::StaticPolarsType>;

    fn new() -> Self {
        <Self as PolarsBuilder<T>>::new()
    }

    fn append(&mut self, val: Option<T>) -> PolarsResult<()> {
        match val {
            Some(t) => PolarsBuilder::<T>::append(self, t),
            None => {
                ChunkedBuilder::append_null(self);
                Ok(())
            }
        }
    }

    fn append_null(&mut self) {
        ChunkedBuilder::append_null(self)
    }

    fn finish(self) -> PolarsResult<ChunkedArray<<Option<T> as MapPolarsType>::StaticPolarsType>> {
        Ok(ChunkedBuilder::finish(self))
    }
}

impl<T> PolarsBuilder<Vec<T>> for AnonymousOwnedListBuilder
where
    T: MapPolarsType,
    Vec<T>: MapPolarsType,
{
    type ChunkedType = ListChunked;

    fn new() -> Self {
        AnonymousOwnedListBuilder::new(PlSmallStr::EMPTY, 0, Some(T::dt()))
    }

    fn append(&mut self, val: Vec<T>) -> PolarsResult<()> {
        let mut ca_builder = T::Builder::new();

        for item in val {
            ca_builder.append(item)?;
        }

        let series = ca_builder.finish()?.into_series();

        self.append_series(&series)
    }

    fn append_null(&mut self) {
        ListBuilderTrait::append_null(self)
    }

    fn finish(mut self) -> PolarsResult<Self::ChunkedType> {
        Ok(ListBuilderTrait::finish(&mut self))
    }
}

impl<T> PolarsBuilder<Option<Vec<T>>> for AnonymousOwnedListBuilder
where
    T: MapPolarsType,
    Vec<T>: MapPolarsType<Builder = Self>,
{
    type ChunkedType = ListChunked;

    fn new() -> Self {
        <AnonymousOwnedListBuilder as PolarsBuilder<Vec<T>>>::new()
    }

    fn append(&mut self, val: Option<Vec<T>>) -> PolarsResult<()> {
        match val {
            Some(t) => PolarsBuilder::<Vec<T>>::append(self, t),

            None => {
                ListBuilderTrait::append_null(self);
                Ok(())
            }
        }
    }

    fn append_null(&mut self) {
        ListBuilderTrait::append_null(self)
    }

    fn finish(self) -> PolarsResult<Self::ChunkedType> {
        PolarsBuilder::<Vec<T>>::finish(self)
    }
}

impl<T, C> PolarsBuilder<Option<T>> for CategoricalChunkedBuilder<C>
where
    Self: PolarsBuilder<T>,
    T: MapPolarsType<Builder = Self>,
    C: PolarsCategoricalType,
{
    type ChunkedType = <Self as PolarsBuilder<T>>::ChunkedType;

    fn new() -> Self {
        <Self as PolarsBuilder<T>>::new()
    }

    fn append(&mut self, val: Option<T>) -> PolarsResult<()> {
        match val {
            Some(v) => <Self as PolarsBuilder<T>>::append(self, v),
            None => {
                self.append_null();
                Ok(())
            }
        }
    }

    fn append_null(&mut self) {
        <Self as PolarsBuilder<T>>::append_null(self)
    }

    fn finish(self) -> PolarsResult<Self::ChunkedType> {
        <Self as PolarsBuilder<T>>::finish(self)
    }
}
