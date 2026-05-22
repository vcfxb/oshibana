//! Macros for generating polars type mapping + matching rust structs

/// Generate record struct type, polars builder type, and polars type mapping
#[macro_export]
macro_rules! generate_record_builder_and_dt {
    {
        $(#[derive($( $derives:ident ),*)])?
        $name:ident $( < $( $gen:tt ),* > )? {
            $(
                // $( #[serde($serde:tt)] )?
                $field:ident: $rt:ty
            ),+
            $(,)?
        }
    } => { paste::paste! {
        $( #[derive( $( $derives ),* )] )?
        pub struct $name $( < $( $gen ),* > )? {
            $(
                // $( #[serde( $serde )] )?
                pub $field: $rt,
            )+
        }

        pub static [< $name:snake:upper _STRUCT_DT >] : ::std::sync::LazyLock<polars::prelude::DataType> = ::std::sync::LazyLock::new(|| {
            use ::polars::prelude::*;

            DataType::Struct(vec![
                $(
                    Field::new(
                        PlSmallStr::from_static( stringify!( $field ) ),
                        <$rt as $crate::traits::map_type::MapPolarsType>::dt()
                    ),
                )+
            ])
        });

        impl $crate::traits::map_type::MapPolarsType for $name {
            type StaticPolarsType = ::polars::datatypes::StructType;
            type Builder = [< $name Builder >] ;

            fn dt() -> ::polars::prelude::DataType {
                [< $name:snake:upper _STRUCT_DT >] .clone()
            }
        }

        pub struct [< $name Builder >] {
            $(
                // pub [< $field _chunk_builder >]: $crate::traits::builder::PolarsBuilder< $rt >,
                pub [< $field _chunk_builder >]: <$rt as $crate::traits::map_type::MapPolarsType>::Builder,

            )+
        }

        impl $crate::traits::builder::PolarsBuilder< $name > for [< $name Builder >] {
            type ChunkedType = ::polars::chunked_array::StructChunked;

            fn new() -> Self {
                Self {
                    $(
                        [< $field _chunk_builder >] : <
                            <$rt as $crate::traits::map_type::MapPolarsType>::Builder
                            as $crate::traits::builder::PolarsBuilder<$rt>
                        >::new(),
                    )+
                }
            }

            fn append(&mut self, val: $name) -> ::polars::prelude::PolarsResult<()> {
                $(
                    $crate::traits::builder::PolarsBuilder::append (
                        &mut self. [< $field _chunk_builder >] ,
                        val. $field
                    )?;
                )+

                Ok(())
            }

            fn append_null(&mut self) {
                $(
                    $crate::traits::builder::PolarsBuilder::append_null (
                        &mut self. [< $field _chunk_builder >]
                    );
                )+
            }

            fn finish(self)
                -> ::polars::prelude::PolarsResult<::polars::chunked_array::StructChunked>
            {
                use ::polars::prelude::*;
                use ::polars::chunked_array::StructChunked;
                use $crate::traits::builder::PolarsBuilder;

                let series = vec![
                    $(
                        PolarsBuilder::<$rt>::finish( self. [< $field _chunk_builder >] )?.into_series(),
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


        impl $crate::traits::builder::PolarsBuilder<Option< $name >> for [< $name Builder >] {
            type ChunkedType = <Self as $crate::traits::builder::PolarsBuilder< $name >>::ChunkedType;

            fn new() -> Self {
                use $crate::traits::builder::PolarsBuilder;
                <Self as PolarsBuilder< $name >>::new()
            }

            fn append(&mut self, val: Option< $name >) -> ::polars::prelude::PolarsResult<()> {
                use $crate::traits::builder::PolarsBuilder;
                match val {
                    Some(v) => <Self as PolarsBuilder< $name >>::append(self, v),
                    None => Ok(<Self as PolarsBuilder< $name >>::append_null(self))
                }
            }

            fn append_null(&mut self) {
                use $crate::traits::builder::PolarsBuilder;
                <Self as PolarsBuilder< $name >>::append_null(self)
            }

            fn finish(self) -> ::polars::prelude::PolarsResult<Self::ChunkedType> {
                use $crate::traits::builder::PolarsBuilder;
                <Self as PolarsBuilder< $name >>::finish(self)
            }
        }
    }};
}
