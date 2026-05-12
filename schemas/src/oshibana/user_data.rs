use std::sync::LazyLock;
use polars::prelude::{Categories, DataType, TimeUnit, TimeZone};
use polars::prelude::Schema;
use strum::{EnumIter, IntoStaticStr};
use crate::oshibana::{enum_to_dt_enum, field, list};
use crate::scryfall::card::finishes::Finish;
use crate::scryfall::card::languages::Language;

#[derive(
    Copy, Clone,
    Debug,
    Default,
    EnumIter, IntoStaticStr
)]
#[strum(serialize_all = "snake_case")]
pub enum CardCondition {
    Mint,
    #[default]
    NearMint,
    LightlyPlayed,
    MediumPlay,
    HeavyPlay,
    Damaged
}

#[derive(
    Debug,
    Copy, Clone,
    PartialEq, Eq,
    EnumIter, IntoStaticStr
)]
#[strum(serialize_all = "lowercase")]
pub enum DeckAction {
    Add,
    Remove
}

pub static COLLECTION_SCHEMA: LazyLock<Schema> = LazyLock::new(|| {
    let global_cats = Categories::global();
    let mapping = global_cats.mapping();
    let global_categorical = DataType::Categorical(global_cats, mapping);

    Schema::from_iter(vec![
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
    ])
});

pub static DECK_SCHEMA: LazyLock<Schema> = LazyLock::new(|| {
    let global_cats = Categories::global();
    let mapping = global_cats.mapping();
    let global_categorical = DataType::Categorical(global_cats, mapping);

    Schema::from_iter(vec![
        field("card_id", DataType::UInt128),
        field("quantity", DataType::UInt32),
        field("tags", list(global_categorical.clone())),
        field("mana_value_override", DataType::String),
        field("board", global_categorical.clone()),
    ])
});

pub static DECK_HISTORY_SCHEMA: LazyLock<Schema> = LazyLock::new(|| {
    let global_cats = Categories::global();
    let mapping = global_cats.mapping();
    let global_categorical = DataType::Categorical(global_cats, mapping);

    Schema::from_iter(vec![
        field("card_id", DataType::UInt128),
        field("quantity", DataType::UInt32),
        field("action", enum_to_dt_enum::<DeckAction>()),
        field("timestamp", DataType::Datetime(TimeUnit::Milliseconds, Some(TimeZone::UTC))),
    ])
});
