use polars::prelude::{DataType, PlSmallStr, StringChunkedBuilder};

pub trait ChunkedArrayBuilder {
    type Builder;
    
    fn dt() -> DataType;

    fn new_builder() -> Self::Builder;

    fn append(builder: &mut Self::Builder, val: Self);
}


impl ChunkedArrayBuilder for String {
    type Builder = StringChunkedBuilder;

    fn dt() -> DataType {
        DataType::String
    }

    fn new_builder() -> Self::Builder {
        StringChunkedBuilder::new(PlSmallStr::EMPTY, 0)
    }

    fn append(builder: &mut Self::Builder, val: Self) {
        builder.append_value(val)
    }
}

impl ChunkedArrayBuilder for Option<String> {
    type Builder = <String as ChunkedArrayBuilder>::Builder;

    fn dt() -> DataType {
        <String as ChunkedArrayBuilder>::dt()
    }
    
    fn new_builder() -> Self::Builder {
        <String as ChunkedArrayBuilder>::new_builder()
    }

    fn append(builder: &mut Self::Builder, val: Self) {
        builder.append_option(val)
    }
}
