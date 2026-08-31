use std::borrow::Borrow;
use std::cmp;
use std::fmt::{Debug, Formatter};
use std::ops::Range;
use std::sync::Arc;

#[derive(Clone, PartialEq, Eq)]
pub struct Fragment {
    pub full_query: Arc<String>,
    pub byte_range: Range<usize>,
}

impl Fragment {
    pub fn as_str(&self) -> &str {
        &self.full_query[self.byte_range.clone()]
    }

    pub fn len(&self) -> usize {
        self.byte_range.end.saturating_sub(self.byte_range.start)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn range(&self) -> &Range<usize> {
        &self.byte_range
    }

    pub fn full_query(&self) -> &str {
        self.full_query.as_ref()
    }

    /// Make a new fragment that covers both of the given fragments from the same source, as well as
    /// anything in between.
    pub fn cover(lhs: &Self, rhs: &Self) -> Self {
        assert!(
            Arc::ptr_eq(&lhs.full_query, &rhs.full_query),
            "fragments must be from same source"
        );

        let start = cmp::min(lhs.byte_range.start, rhs.byte_range.start);
        let end = cmp::max(lhs.byte_range.end, rhs.byte_range.end);

        Fragment {
            full_query: Arc::clone(&lhs.full_query),
            byte_range: start..end,
        }
    }
}

impl Borrow<str> for Fragment {
    fn borrow(&self) -> &str {
        &self.full_query[self.byte_range.clone()]
    }
}

impl Debug for Fragment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_str())
    }
}
