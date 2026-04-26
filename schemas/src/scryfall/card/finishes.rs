use serde::{Deserialize, Serialize};

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Finish {
    Foil,
    Nonfoil,
    Etched,
}
