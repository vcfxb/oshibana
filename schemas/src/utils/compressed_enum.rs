//! Serialization wrapper for serializing an enum as its discriminant.

use enum_ordinalize::Ordinalize;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Debug;
use typename::TypeName;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct AsDiscriminant<T>(pub T);

impl<T> Serialize for AsDiscriminant<T>
where
    T: Ordinalize,
    <T as Ordinalize>::VariantType: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ord = self.0.ordinal();
        ord.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for AsDiscriminant<T>
where
    T: Ordinalize + TypeName,
    <T as Ordinalize>::VariantType: Deserialize<'de> + Debug + Copy,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let ord = <<T as Ordinalize>::VariantType as Deserialize>::deserialize(deserializer)?;

        T::from_ordinal(ord)
            .map(AsDiscriminant)
            .ok_or(D::Error::custom(format!(
                "expected {} got {ord:?}",
                T::type_name()
            )))
    }
}
