use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};

#[derive(
    Copy, Clone,
    Debug,
    Eq, PartialEq,
    Deserialize, Serialize,
    EnumIter, IntoStaticStr
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Special,
    Mythic,
    Bonus,
}
