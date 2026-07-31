//! Parser for scryfall-syntax queries

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
    full_query: &'i str,
    tokens: Vec<Token<'i>>,
    idx: usize,
    pub diagnostics: Vec<Diagnostic<'i>>
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
                full_query: query_str,
                tokens,
                idx: 0,
                diagnostics: Vec::new(),
            },

            Err(message) => Self {
                full_query: query_str,
                tokens: Vec::new(),
                idx: 0,
                diagnostics: vec![Diagnostic::Error { message: message.into(), fragment: None }],
            }
        }
    }

    pub fn bytes_consumed(&self) -> usize {
        self.tokens[..self.idx].iter().map(|t| t.frag.len()).sum()
    }

    pub fn bytes_remaining(&self) -> usize {
        self.tokens[self.idx..].iter().map(|t| t.frag.len()).sum()
    }

    pub fn parse_query(&mut self) -> Union<'i> {
        Union::parse(self)
    }

    pub fn exhausted(&self) -> bool {
        self.idx >= self.tokens.len()
    }

    /// Peek the next non whitespace token.
    pub fn peek(&self) -> Option<&Token<'i>> {
        self.peek_n(0)
    }
    
    /// Peek the `n`th non-whitespace token ahead (0 is the immediate next).
    pub fn peek_n(&self, mut n: usize) -> Option<&Token<'i>> {
        let mut offset = 0;
        while let Some(token) = self.tokens.get(self.idx + offset) {
            if token.kind != TokenTy::Whitespace {
                if n == 0 {
                    return Some(token);
                }
                n -= 1;
            }
            offset += 1;
        }
        None
    }

    /// Advance past current whitespaces and consume the next non-whitespace token.
    pub fn pull(&mut self) -> Option<&Token<'i>> {
        while let Some(token) = self.tokens.get(self.idx) {
            self.idx += 1;
            if token.kind != TokenTy::Whitespace {
                return Some(token);
            }
        }
        None
    }

    /// Advance to the next non-whitespace token if it's of the given `kind`.
    pub fn next_if_is(&mut self, kind: TokenTy) -> Option<&Token<'i>> {
        match self.peek() {
            Some(token) if token.kind == kind => self.pull(),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use schemas::scryfall::card::languages::Language;
    use crate::scryfall::search::query_parser::filter::Filter;
    use crate::scryfall::search::query_parser::item::ItemInner;
    use crate::scryfall::search::query_parser::Parser;

    #[test]
    fn parse_language_filter() {
        let mut parser = Parser::new("lang:en");
        let query = parser.parse_query();
        let item = &query.intersections[0].items[0];
        assert_eq!(item.modifier, None);
        assert_eq!(item.cover.as_str(), "");
        let ItemInner::Filter(ref filter) = item.inner else {
            panic!("is not a filter");
        };
        let Filter::Lang { value } = filter else {
            panic!("is not a language filter");
        };

        assert_eq!(*value, Language::En);
    }
}

