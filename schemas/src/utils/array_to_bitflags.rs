use enumflags2::{BitFlag, BitFlags};
use serde::de::{Error, SeqAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt::Formatter;
use std::marker::PhantomData;
use strum::IntoEnumIterator;

pub trait ExpectStr {
    const EXPECT_STR: &'static str;
}

/// Structure that deserializes an array of string enum values into a bitset.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ArrayToBitset<T: Into<&'static str> + IntoEnumIterator + BitFlag + ExpectStr> {
    pub bitflags: BitFlags<T>,
}

impl<'de, T> Deserialize<'de> for ArrayToBitset<T>
where
    T: Into<&'static str> + IntoEnumIterator + BitFlag + ExpectStr,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_seq(AnonVisitor::<T> {
                _phantom: PhantomData,
            })
            .map(|bitflags| ArrayToBitset { bitflags })
    }
}

struct AnonVisitor<I> {
    _phantom: PhantomData<I>,
}

impl<'de, I> Visitor<'de> for AnonVisitor<I>
where
    I: BitFlag + ExpectStr + IntoEnumIterator + Into<&'static str>,
{
    type Value = BitFlags<I>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str(<I as ExpectStr>::EXPECT_STR)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut bitflags: BitFlags<I> = BitFlags::<I>::empty();

        'seq: while let Some(next) = seq.next_element::<&str>()? {
            for variant in <I as IntoEnumIterator>::iter() {
                let variant_str: &'static str = variant.into();
                if variant_str.eq_ignore_ascii_case(next) {
                    bitflags |= variant;
                    continue 'seq;
                }
            }

            return Err(Error::invalid_value(Unexpected::Str(next), &"frame effect"));
        }

        Ok(bitflags)
    }
}
