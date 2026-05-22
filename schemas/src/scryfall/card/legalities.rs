use polars::datatypes::Categorical8Type;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use typename::TypeName;
use crate::{generate_enum_dt_map_and_builder_impl, generate_record_builder_and_dt};

#[derive(
    Deserialize,
    Debug,
    Copy,
    Clone,
    Hash,
    Eq,
    PartialEq,
    Serialize,
    TypeName,
    IntoStaticStr,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Legality {
    Legal,
    NotLegal,
    Restricted,
    Banned,
}

generate_enum_dt_map_and_builder_impl!(Legality => Categorical8Type);

generate_record_builder_and_dt! {
    #[derive(Serialize, Deserialize, Copy, Clone, Debug)]
    Legalities {
        standard: Legality,
        historic: Legality,
        timeless: Legality,
        pioneer: Legality,
        modern: Legality,
        legacy: Legality,
        pauper: Legality,
        vintage: Legality,
        commander: Legality,
        oathbreaker: Legality,
        standardbrawl: Legality,
        brawl: Legality,
        alchemy: Legality,
        paupercommander: Legality,
        duel: Legality,
        oldschool: Legality,
        premodern: Legality,
    }
}
