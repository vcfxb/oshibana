//! Parser for scryfall-syntax queries

use polars::polars_utils::collection::Collection;
use crate::scryfall::search::query_parser::fragment::Fragment;
use crate::scryfall::search::query_parser::lexer::{Lexer, Token, TokenTy};
use crate::scryfall::search::query_parser::union::Union;

pub mod filter;
pub mod group;
pub mod intersection;
pub mod item;
pub mod operator;
pub mod union;
pub mod fragment;
pub mod lexer;

pub struct Parser<'i> {
    tokens: Vec<Token<'i>>,
    idx: usize,
    diagnostics: Vec<Diagnostic<'i>>
}

#[derive(Debug)]
pub enum Diagnostic<'i> {
    Error {
        message: String,
        fragment: Option<Fragment<'i>>
    },

    Warning {
        message: String,
        fragment: Fragment<'i>,
    }
}

impl<'i> Parser<'i> {
    pub fn new(query_str: &'i str) -> Self {
        match Lexer::lex(query_str) {
            Ok(tokens) => Self {
                tokens,
                idx: 0,
                diagnostics: Vec::new(),
            },

            Err(message) => Self {
                tokens: Vec::new(),
                idx: 0,
                diagnostics: vec![Diagnostic::Error { message: message.into(), fragment: None }],
            }
        }
    }

    pub fn parse_query(&mut self) -> Union<'i> {
        unimplemented!()
    }

    fn exhausted(&self) -> bool {
        self.idx >= self.tokens.len()
    }

    /// Peek the next non whitespace token.
    fn peek(&self) -> Option<&Token<'i>> {
        let mut offset = 0;
        while let Some(token) = self.tokens.get(self.idx + offset) {
            if token.kind != TokenTy::Whitespace {
                return Some(token);
            }

            offset += 1;
        }

        None
    }

    /// Advance to the next non whitespace token.
    fn advance(&mut self) -> Option<&Token<'i>> {
        while let Some(token) = self.tokens.get(self.idx) {
            if token.kind != TokenTy::Whitespace {
                return Some(token);
            }

            self.idx += 1;
        }

        None
    }

    /// Advance to the next non-whitespace token if it's of the given `kind`.
    fn next_if_is(&mut self, kind: TokenTy) -> Option<&Token<'i>> {
        match self.peek() {
            Some(token) if token.kind == kind => self.advance(),
            _ => None,
        }
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
