use crate::oshibana::{enum_to_dt_enum, field, list};
use crate::scryfall::card::finishes::Finish;
use crate::scryfall::card::languages::Language;
use polars::prelude::{Schema, SchemaRef};
use polars::prelude::{Categories, DataType, TimeUnit, TimeZone};
use std::sync::{Arc, LazyLock};
use strum::{EnumIter, IntoStaticStr};

#[derive(Copy, Clone, Debug, Default, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum CardCondition {
    Mint,
    #[default]
    NearMint,
    LightlyPlayed,
    MediumPlay,
    HeavyPlay,
    Damaged,
}

pub static COLLECTION_SCHEMA: LazyLock<SchemaRef> = LazyLock::new(|| {
    let global_cats = Categories::global();
    let mapping = global_cats.mapping();
    let global_categorical = DataType::Categorical(global_cats, mapping);
    let timestamp_dt = DataType::Datetime(TimeUnit::Milliseconds, Some(TimeZone::UTC));

    let schema = Schema::from_iter(vec![
        field("card_id", DataType::UInt128),
        field("quantity", DataType::UInt32),
        field("binder", global_categorical.clone()),
        // price in cents as an integer to avoid float errors
        field("price_cents", DataType::UInt32),
        field("note", DataType::String),
        field("is_proxy", DataType::Boolean),
        field("is_alter", DataType::Boolean),
        field("deck_assignment", global_categorical.clone()),
        field("condition", enum_to_dt_enum::<CardCondition>()),
        field("language", enum_to_dt_enum::<Language>()),
        field("finish", enum_to_dt_enum::<Finish>()),
        field("added_on", timestamp_dt.clone()),
        field("updated_on", timestamp_dt.clone()),
    ]);

    Arc::new(schema)
});

pub static DECK_SCHEMA: LazyLock<SchemaRef> = LazyLock::new(|| {
    let global_cats = Categories::global();
    let mapping = global_cats.mapping();
    let global_categorical = DataType::Categorical(global_cats, mapping);

    let schema = Schema::from_iter(vec![
        field("deck", global_categorical.clone()),
        field("card_id", DataType::UInt128),
        field("quantity", DataType::UInt32),
        field("tags", list(global_categorical.clone())),
        field("mana_value_override", DataType::String),
        field("board", global_categorical.clone()),
    ]);

    Arc::new(schema)
});

pub static DECK_HISTORY_SCHEMA: LazyLock<Schema> = LazyLock::new(|| {
    let global_cats = Categories::global();
    let mapping = global_cats.mapping();
    let global_categorical = DataType::Categorical(global_cats, mapping);
    let timestamp_dt = DataType::Datetime(TimeUnit::Milliseconds, Some(TimeZone::UTC));

    Schema::from_iter(vec![
        field("deck", global_categorical.clone()),
        field("card_id", DataType::UInt128),
        field("quantity_delta", DataType::Int64),
        field("timestamp", timestamp_dt),
    ])
});
