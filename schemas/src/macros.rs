//! Macros for generating polars schemas + matching rust structs


/// Generate record struct type, ChunkedBuilder type, and polars Schema type
#[macro_export]
macro_rules! generate_record_builder_and_dt {
    {
        $(#[derive($( $derives:ident ),*)])?
        $name:ident $( < $( $gen:tt ),* > )? {
            $(
                $( #[serde($serde:tt)] )?
                $field:ident: $t:ty
            ),+
            $(,)?
        }
    } => { paste::paste! {
        $( #[derive( $( $derives ),* )] )?
        pub struct $name $( < $( $gen ),* > )? {
            $(
                $( #[serde( $serde )] )?
                pub $field: $t,
            )+
        }

        pub struct [< $name Builder >] {
            $(
                pub [< $field _chunk_builder >]: <$t as $crate::traits::ChunkedArrayBuilder>::Builder,
            )+
        }

        impl [< $name Builder >] {
            pub fn new() -> Self {
                Self {
                    $(
                        [< $field _chunk_builder >] : <$t as $crate::traits::ChunkedArrayBuilder>::new_builder(),
                    )+
                }
            }

            pub fn append(&mut self, val: $name) {
                $(
                    $crate::traits::ChunkedArrayBuilder::append (
                        &mut self. [< $field _chunk_builder >] ,
                        val. $field
                    );
                )+
            }

            pub fn finish(mut self)
                -> Result<
                    ::polars::prelude::ChunkedArray<::polars::prelude::StructType>,
                    ::polars::prelude::PolarsError
                >
            {
                use ::polars::prelude::*;
                use ::polars::chunked_array::StructChunked;

                let series = vec![
                    $(
                        self. [< $field _chunk_builder >] .finish().into_series(),
                    )+
                ];

                let len = series[0].len();

                StructChunked::from_series(
                    PlSmallStr::EMPTY,
                    len,
                    series.iter()
                )
            }
        }

        pub static [< $name:snake:upper _STRUCT_DT >] : ::std::sync::LazyLock<polars::prelude::DataType> = ::std::sync::LazyLock::new(|| {
            use ::polars::prelude::*;

            DataType::Struct(vec![
                $(
                    Field::new(
                        PlSmallStr::from_static( stringify!( $field ) ),
                        <$t as $crate::traits::ChunkedArrayBuilder>::dt()
                    ),
                )+
            ])
        });
    }};
}

macro_rules! impl_ca_builder_for_prim {
    ($t:ty) => {
        impl ChunkedArrayBuilder for $t {
            type Builder = polars::prelude::PrimitiveChunkedBuilder<<$t as polars::prelude::NumericNative>::PolarsType>;

            fn dt() -> ::polars::prelude::DataType {
                use polars::prelude::*;
                <<$t as NumericNative>::PolarsType as PolarsDataType>::get_static_dtype()
            }

            fn new_builder() -> Self::Builder {
                use polars::prelude::*;
                PrimitiveChunkedBuilder::new(PlSmallStr::EMPTY, 0)
            }

            fn append(builder: &mut Self::Builder, val: Self) {
                use polars::prelude::ChunkedBuilder;
                builder.append_value(val)
            }
        }

        impl ChunkedArrayBuilder for Option<$t> {
            type Builder = <$t as ChunkedArrayBuilder>::Builder;

            fn dt() -> DataType {
                <$t as ChunkedArrayBuilder>::dt()
            }

            fn new_builder() -> Self::Builder {
                <$t as ChunkedArrayBuilder>::new_builder()
            }

            fn append(builder: &mut Self::Builder, val: Self) {
                use polars::prelude::ChunkedBuilder;
                builder.append_option(val)
            }
        }

        impl ChunkedArrayBuilder for Vec<$t> {
            type Builder = polars::prelude::ListPrimitiveChunkedBuilder<<$t as polars::prelude::NumericNative>::PolarsType>;

            fn dt() -> DataType {
                use polars::prelude::*;
                DataType::List(Box::new(<$t as ChunkedArrayBuilder>::dt()))
            }

            fn new_builder() -> Self::Builder {
                use polars::prelude::*;

                let dt = <<$t as NumericNative>::PolarsType as PolarsDataType>::get_static_dtype();

                ListPrimitiveChunkedBuilder::new(
                    PlSmallStr::EMPTY,
                    0,
                    0,
                    dt
                )
            }

            fn append(builder: &mut Self::Builder, val: Self) {
                builder.append_slice(val.as_slice())
            }
        }

        impl ChunkedArrayBuilder for Option<Vec<$t>> {
            type Builder = <Vec<$t> as ChunkedArrayBuilder>::Builder;

            fn dt() -> DataType {
                <Vec<$t> as ChunkedArrayBuilder>::dt()
            }

            fn new_builder() -> Self::Builder {
                <Vec<$t> as ChunkedArrayBuilder>::new_builder()
            }

            fn append(builder: &mut Self::Builder, val: Self) {
                builder.append_opt_slice(val.as_ref().map(Vec::as_slice))
            }
        }
    };

    ($($t:ty,)+) => {
        $( impl_ca_builder_for_prim!($t); )+
    }
}

use polars::datatypes::DataType;
use crate::traits::ChunkedArrayBuilder;
impl_ca_builder_for_prim!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f32, f64, );
