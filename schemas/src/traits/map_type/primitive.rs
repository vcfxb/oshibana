use crate::traits::{
    map_type::MapPolarsType,
    builder::PolarsBuilder
};
use polars::prelude::*;

macro_rules! impl_map_to_polars_for_prim {
    ($t:ty) => {
        impl MapPolarsType for $t {
            type StaticPolarsType = <$t as polars::prelude::NumericNative>::PolarsType;
            type Builder = PrimitiveChunkedBuilder<Self::StaticPolarsType>;

            fn dt() -> DataType {
                <<$t as NumericNative>::PolarsType as PolarsDataType>::get_static_dtype()
            }
        }

        impl PolarsBuilder<$t> for PrimitiveChunkedBuilder<<$t as MapPolarsType>::StaticPolarsType> {
            type ChunkedType = ChunkedArray<<$t as MapPolarsType>::StaticPolarsType>;

            fn new() -> Self {
                PrimitiveChunkedBuilder::new(PlSmallStr::EMPTY, 0)
            }

            fn append(&mut self, val: $t) -> PolarsResult<()> {
                ChunkedBuilder::append_value(self, val);
                Ok(())
            }
            
            fn append_null(&mut self) {
                ChunkedBuilder::append_null(self)
            }

            fn finish(self) -> PolarsResult<ChunkedArray<<$t as MapPolarsType>::StaticPolarsType>> {
                Ok(ChunkedBuilder::finish(self))
            }
        }

    };

    ($($t:ty,)+) => {
        $( impl_map_to_polars_for_prim!($t); )+
    }
}

impl_map_to_polars_for_prim!(u8, i8, u16, i16, f32, i32, u32, f64, i64, u64, u128, i128, );

// impl MapPolarsType for u8 {
//     type StaticPolarsType = UInt8Type;
//     type Builder = PrimitiveChunkedBuilder<UInt8Type>;
//
//     fn dt() -> DataType {
//         DataType::UInt8
//     }
// }
//
// impl PolarsBuilder<u8> for PrimitiveChunkedBuilder<UInt8Type> {
//     type ChunkedType = ChunkedArray<UInt8Type>;
//
//     fn new() -> Self {
//         todo!()
//     }
//
//     fn append(&mut self, val: u8) -> PolarsResult<()> {
//         todo!()
//     }
//
//     fn finish(self) -> PolarsResult<Self::ChunkedType> {
//         todo!()
//     }
// }
//
// impl MapPolarsType for Vec<u8> {
//     type StaticPolarsType = ListType;
//     type Builder = ListPrimitiveChunkedBuilder<UInt8Type>;
//
//     fn dt() -> DataType {
//         DataType::List(Box::new(DataType::UInt8))
//     }
// }
