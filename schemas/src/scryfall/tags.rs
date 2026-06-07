//! As of June 2026, scryfall also exposes tagger data to us

use crate::{generate_enum_dt_map_and_builder_impl, generate_record_builder_and_dt};
use polars::prelude::{Categorical8Type, DataType, Schema, SchemaRef};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, LazyLock};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;
use url::Url;
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
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TagWeight {
    VeryStrong,
    Strong,
    Median,
    Weak,
}

generate_enum_dt_map_and_builder_impl!(TagWeight => Categorical8Type);

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
pub enum TagType {
    Illustration,
    Oracle,
}

generate_enum_dt_map_and_builder_impl!(TagType => Categorical8Type);

generate_record_builder_and_dt! {
    #[derive(Deserialize, Serialize, Debug)]
    Tagging {
        illustration_id: Option<Uuid>,
        oracle_id: Option<Uuid>,
        weight: TagWeight,
        annotation: Option<String>
    }
}

generate_record_builder_and_dt! {
    #[derive(Deserialize, Serialize, Debug)]
    ScryfallTag {
        id: Uuid,
        slug: String,
        label: String,
        uri: Url,
        r#type: TagType,
        description: Option<String>,
        parent_ids: Option<Vec<Uuid>>,
        child_ids: Option<Vec<Uuid>>,
        aliases: Option<Vec<String>>,
        taggings: Vec<Tagging>
    }
}

pub static SCRYFALL_TAGS_SCHEMA: LazyLock<SchemaRef> = LazyLock::new(|| {
    let DataType::Struct(fields) = &*SCRYFALL_TAG_STRUCT_DT else {
        unreachable!("the tags struct dt is defined to be a struct");
    };

    Arc::new(Schema::from_iter(fields.iter().cloned()))
});
