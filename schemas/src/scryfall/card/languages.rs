use enumflags2::bitflags;
use serde::Deserialize;

#[bitflags]
#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(u32)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// English
    En,
    /// Spanish
    Es,
    /// French
    Fr,
    /// German
    De,
    /// Italian
    It,
    /// Portuguese
    Pt,
    /// Japanese
    Ja,
    /// Korean
    Ko,
    /// Russian
    Ru,
    /// Simplified Chinese
    Zhs,
    /// Traditional Chinese
    Zht,
    /// Hebrew
    He,
    /// Latin
    La,
    /// Ancient Greek
    Grc,
    /// Arabic
    Ar,
    /// Sanskrit
    Sa,
    /// Phyrexian
    Ph,
    /// Quenya -- lotr language
    Qya,
}
