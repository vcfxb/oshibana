use std::borrow::Borrow;
use std::ops::Range;

#[derive(Debug)]
pub struct Fragment<'i> {
    full_query: &'i str,
    byte_range: Range<usize>,
}

impl<'i> Fragment<'i> {
    pub const fn new(full_query: &'i str) -> Self {
        Fragment { full_query, byte_range: 0..full_query.len() }
    }

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
