//! Deserialize strings that exactly match the parameterized const.

use serde::de::{Error, Unexpected, Visitor};
use std::fmt;

/// Deserialize successfully only if the input matches a specific string.
pub struct DeserializeMatches(pub &'static str);

impl<'de> Visitor<'de> for DeserializeMatches {
    type Value = &'static str;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string matching exactly \"{}\"", self.0)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if v == self.0 {
            Ok(self.0)
        } else {
            Err(Error::invalid_value(Unexpected::Str(v), &self))
        }
    }
}
