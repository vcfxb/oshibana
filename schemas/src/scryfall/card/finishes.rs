use serde::Deserialize;

#[derive(Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Finish {
    Foil,
    Nonfoil,
    Etched,
}
