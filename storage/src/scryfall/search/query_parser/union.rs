use pest::iterators::Pair;
use polars::prelude::Expr;
use crate::scryfall::search::polars_mapping::MapToPolarsExpr;
use crate::scryfall::search::query_parser::intersection::Intersection;
use crate::scryfall::search::query_parser::Rule;

pub struct Union<'i> {
    pub intersections: Vec<Intersection<'i>>
}

impl<'i> Union<'i> {
    pub(in super) fn consume(pair: Pair<'i, Rule>) -> Self {
        assert_eq!(pair.as_rule(), Rule::union, "{:?} is not a union", pair.as_rule());
        
        Self {
            intersections: pair.into_inner()
                .map(Intersection::consume)
                .collect()
        }
    }
}

impl<'i> MapToPolarsExpr for Union<'i> {
    fn as_pexpr(&self) -> Expr {
        use polars::prelude::*;
        
        self.intersections
            .iter()
            .fold(lit(false), |acc, item| acc.or(item.as_pexpr()))
    }
}
