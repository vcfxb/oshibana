use crate::scryfall::search::polars_mapping::MapToPolarsExpr;
use crate::scryfall::search::query_parser::Rule;
use crate::scryfall::search::query_parser::item::Item;
use pest::iterators::Pair;
use polars::prelude::Expr;
use crate::scryfall::search::query_parser::fragment::Fragment;

pub struct Intersection<'i> {
    pub items: Vec<Item<'i>>,
}

impl<'i> Intersection<'i> {
    pub(super) fn consume(pair: Pair<'i, Rule>) -> Self {
        assert_eq!(
            pair.as_rule(),
            Rule::intersection,
            "{:?} is not an intersection",
            pair.as_rule()
        );

        Self {
            items: pair.into_inner().map(Item::consume).collect(),
        }
    }
    
    pub fn fragment(&self) -> Fragment<'i> {
        match (self.items.first(), self.items.last()) {
            (Some(first), Some(last)) => Fragment::cover(first)
        }
    }
}

impl<'i> MapToPolarsExpr for Intersection<'i> {
    fn as_pexpr(&self) -> Expr {
        use polars::prelude::*;

        self.items
            .iter()
            .fold(lit(true), |acc, item| acc.and(item.as_pexpr()))
    }
}
