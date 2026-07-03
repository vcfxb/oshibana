//! Parser for scryfall-syntax queries

pub mod filter;
pub mod group;
pub mod intersection;
pub mod item;
pub mod operator;
pub mod union;

use crate::scryfall::search::polars_mapping::MapToPolarsExpr;
use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use polars::prelude::Expr;
use union::Union;

#[derive(Parser)]
#[grammar = "scryfall/search/query_grammar.pest"]
struct QueryParser;

pub struct Query<'i> {
    pub union: Union<'i>,
}

impl<'i> Query<'i> {
    pub fn parse(input: &'i str) -> Result<Option<Self>, pest::error::Error<Rule>> {
        let mut query_pairs = QueryParser::parse(Rule::query, input)?;
        let Some(query_pair) = query_pairs.next() else {
            return Ok(None);
        };

        let union_pair = query_pair.into_inner().next().unwrap();
        let query = Self {
            union: Union::consume(union_pair),
        };
        assert_eq!(query_pairs.next(), None);
        Ok(Some(query))
    }
}

impl<'i> MapToPolarsExpr for Query<'i> {
    fn as_pexpr(&self) -> Expr {
        self.union.as_pexpr()
    }
}

fn unwrap_exactly_one(pair: Pair<Rule>) -> Pair<Rule> {
    let mut inner = pair.into_inner();
    let result = inner.next().expect("pair is not empty");
    assert_eq!(inner.next(), None, "more than one pair found");
    result
}

#[cfg(test)]
mod tests {
    use crate::scryfall::search::query_parser::Query;

    #[test]
    fn parse_language_filter() -> anyhow::Result<()> {
        let parse = Query::parse("lang:en")?.unwrap();

        Ok(())
    }
}
