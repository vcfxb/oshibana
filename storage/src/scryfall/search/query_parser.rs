//! Parser for scryfall-syntax queries

use polars::polars_utils::collection::Collection;
use crate::scryfall::search::query_parser::lexer::{Token, TokenTy};

pub mod filter;
pub mod group;
pub mod intersection;
pub mod item;
pub mod operator;
pub mod union;
pub mod fragment;
pub mod lexer;
pub mod query;

pub struct Parser<'i> {
    tokens: Vec<Token<'i>>,
    idx: usize,

}

impl<'i> Parser<'i> {
    fn head(&self) -> Option<&Token<'i>> {
        self.peek_n(0)
    }

    fn head_kind(&self) -> Option<TokenTy> {
        self.head().map(|t| t.kind)
    }

    fn peek(&self) -> Option<&Token<'i>> {
        self.peek_n(1)
    }

    fn peek_kind(&self) -> Option<TokenTy> {
        self.peek().map(|t| t.kind)
    }

    /// Peek several tokens forward, 0 being head.
    fn peek_n(&self, idx: usize) -> Option<&Token<'i>> {
        self.tokens.get(self.idx + idx)
    }

    /// Peek several tokens at once.
    fn peek_multi(&self, n: usize) -> Option<&[Token<'i>]> {
        if self.idx + n <= self.tokens.len() {
            Some(&self.tokens[self.idx..self.idx + n])
        } else {
            None
        }
    }

    fn advance(&mut self, count: usize) {
        self.idx += count;
    }

    fn pull(&mut self) -> &Token<'i> {
        self.idx += 1;
        &self.tokens[self.idx - 1]
    }
}


// #[derive(Parser)]
// #[grammar = "scryfall/search/query_grammar.pest"]
// struct QueryParser;
// 
// pub struct Query<'i> {
//     pub union: Union<'i>,
// }

// impl<'i> Query<'i> {
//     pub fn parse(input: &'i str) -> Result<Option<Self>, pest::error::Error<Rule>> {
//         let mut query_pairs = QueryParser::parse(Rule::query, input)?;
//         let Some(query_pair) = query_pairs.next() else {
//             return Ok(None);
//         };
// 
//         let union_pair = query_pair.into_inner().next().unwrap();
//         let query = Self {
//             union: Union::consume(union_pair),
//         };
//         assert_eq!(query_pairs.next(), None);
//         Ok(Some(query))
//     }
// }
// 
// impl<'i> MapToPolarsExpr for Query<'i> {
//     fn as_pexpr(&self) -> Expr {
//         self.union.as_pexpr()
//     }
// }
// 
// fn unwrap_exactly_one(pair: Pair<Rule>) -> Pair<Rule> {
//     let mut inner = pair.into_inner();
//     let result = inner.next().expect("pair is not empty");
//     assert_eq!(inner.next(), None, "more than one pair found");
//     result
// }
// 
// #[cfg(test)]
// mod tests {
//     use crate::scryfall::search::query_parser::Query;
// 
//     #[test]
//     fn parse_language_filter() -> anyhow::Result<()> {
//         let parse = Query::parse("lang:en")?.unwrap();
// 
//         Ok(())
//     }
// }
