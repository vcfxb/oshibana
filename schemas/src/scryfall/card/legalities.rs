use serde::Deserialize;

#[derive(Deserialize, Debug, Copy, Clone, Hash, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Legality {
    Legal,
    NotLegal,
    Restricted,
    Banned,
}
