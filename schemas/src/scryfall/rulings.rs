use crate::{generate_enum_dt_map_and_builder_impl, generate_record_builder_and_dt};
use chrono::NaiveDate;
use polars::datatypes::DataType;
use polars::prelude::{Categorical8Type, Schema, SchemaRef};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, LazyLock};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;
use uuid::Uuid;

#[derive(
    Deserialize,
    Copy,
    Clone,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    TypeName,
    EnumIter,
    IntoStaticStr,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum RulingSource {
    Scryfall,
    Wotc,
}

generate_enum_dt_map_and_builder_impl!(RulingSource => Categorical8Type);

generate_record_builder_and_dt! {
    #[derive(Debug, Deserialize, Serialize)]
    ScryfallRuling {
        oracle_id: Uuid,
        source: RulingSource,
        published_at: NaiveDate,
        comment: String
    }
}

pub static SCRYFALL_RULING_SCHEMA: LazyLock<SchemaRef> = LazyLock::new(|| {
    let DataType::Struct(fields) = &*SCRYFALL_RULING_STRUCT_DT else {
        unreachable!("the ruling struct dt is defined to be a struct");
    };

    Arc::new(Schema::from_iter(fields.iter().cloned()))
});
