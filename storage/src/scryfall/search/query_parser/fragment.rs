use std::borrow::Borrow;
use std::ops::Range;
use std::{cmp, ptr};
use std::fmt::{Debug, Formatter};

#[derive(Clone, PartialEq, Eq)]
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
    
    /// Make a new fragment that covers both of the given fragments from the same source, as well as
    /// anything in between.
    pub fn cover(lhs: &Self, rhs: &Self) -> Self {
        assert!(ptr::eq(lhs.full_query, rhs.full_query), "fragments must be from same source");
        
        let start = cmp::min(lhs.byte_range.start, rhs.byte_range.start);
        let end = cmp::max(lhs.byte_range.end, rhs.byte_range.end);
        
        Fragment {
            full_query: lhs.full_query,
            byte_range: start..end,
        }
    }
}

impl<'i> Borrow<str> for Fragment<'i> {
    fn borrow(&self) -> &str {
        &self.full_query[self.byte_range.clone()]
    }
}

impl<'i> Debug for Fragment<'i> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}
