use crate::generate_enum_dt_map_and_builder_impl;
use polars::datatypes::Categorical8Type;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;

#[derive(
    Deserialize,
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    TypeName,
    IntoStaticStr,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Layout {
    Normal,
    Split,
    Flip,
    Transform,
    ModalDfc,
    Meld,
    Leveler,
    Class,
    Case,
    Saga,
    Adventure,
    Prepare,
    Mutate,
    Prototype,
    Battle,
    Planar,
    Scheme,
    Vanguard,
    Token,
    DoubleFacedToken,
    Emblem,
    Augment,
    Host,
    ArtSeries,
    ReversibleCard,
    // (2026 Aug 15): Scryfall seems to have added this, not sure what for
    FrontCard
}

generate_enum_dt_map_and_builder_impl!(Layout => Categorical8Type);
