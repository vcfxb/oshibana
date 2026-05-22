use crate::{generate_enum_dt_map_and_builder_impl, generate_record_builder_and_dt};
use polars::prelude::Categorical8Type;
use serde::{Deserialize, Serialize};
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
pub enum Component {
    Token,
    MeldPart,
    MeldResult,
    ComboPiece,
}

generate_enum_dt_map_and_builder_impl!(Component => Categorical8Type);

generate_record_builder_and_dt! {
    #[derive(Deserialize, Debug, Serialize)]
    RelatedCard {
        id: Uuid,
        component: Component,
        name: String,
        type_line: String,
        uri: Url,
    }
}
