use std::borrow::Borrow;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fragment<'i> {
    pub full_query: &'i str,
    pub byte_range: Range<usize>,
}

impl<'i> Fragment<'i> {
    pub fn as_str(&self) -> &'i str {
        &self.full_query[self.byte_range.clone()]
    }

    pub fn len(&self) -> usize {
        self.byte_range.end - self.byte_range.start
    }

    pub fn range(&self) -> &Range<usize> {
        &self.byte_range
    }

    pub fn full_query(&self) -> &'i str {
        self.full_query
    }
}

impl<'i> Borrow<str> for Fragment<'i> {
    fn borrow(&self) -> &str {
        &self.full_query[self.byte_range.clone()]
    }
}
