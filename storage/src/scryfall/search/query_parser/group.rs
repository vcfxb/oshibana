use pest::iterators::Pair;
use polars::prelude::Expr;
use crate::scryfall::search::polars_mapping::MapToPolarsExpr;
use crate::scryfall::search::query_parser::{unwrap_exactly_one, Rule, Union};

pub struct Group<'i> {
    pub inner: Box<Union<'i>>
}

impl<'i> Group<'i> {
    pub(in super) fn consume(pair: Pair<'i, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::group, "{:?} is not a group", pair.as_rule());
        Self { inner: Box::new(Union::consume(unwrap_exactly_one(pair))) }
    }
}

impl<'i> MapToPolarsExpr for Group<'i> {
    fn as_pexpr(&self) -> Expr {
        self.inner.as_pexpr()
    }
}
