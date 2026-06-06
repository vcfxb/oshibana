use polars::prelude::DataType;
use schemas::scryfall::card::SCRYFALL_CARD_SCHEMA;

pub fn get_possible_columns() -> Vec<String> {
    let mut result = Vec::new();

    for (name, dt) in SCRYFALL_CARD_SCHEMA.iter() {
        if !dt.is_struct() {
            result.push(name.to_string());
            continue;
        } else {
            result.append(&mut unpack_struct(name.as_str(), dt));
        }
    }

    result
}

fn unpack_struct(prefix: &str, dt: &DataType) -> Vec<String> {
    let DataType::Struct(fields) = dt else {
        panic!("dt is not struct");
    };
    let mut result = Vec::new();

    for field in fields {
        if field.dtype.is_struct() {
            let prefix = prefix.to_string() + "." + field.name.as_str();
            result.append(&mut unpack_struct(&prefix, &field.dtype));
        } else {
            result.push(prefix.to_string() + "." + field.name());
        }
    }

    result
}
