use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SecurityStamp {
    Oval,
    Triangle,
    Acorn,
    Circle,
    Arena,
    Heart,
}
