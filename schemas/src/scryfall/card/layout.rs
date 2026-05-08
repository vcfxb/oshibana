use enum_ordinalize::Ordinalize;
use serde::{Deserialize, Serialize};
use typename::TypeName;

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Ordinalize, TypeName)]
#[serde(rename_all = "snake_case")]
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
}
