use crate::scryfall::search::polars_mapping::MapToPolarsExpr;
use crate::scryfall::search::query_parser::{Rule, Union, unwrap_exactly_one, Parser, Diagnostic};
use pest::iterators::Pair;
use polars::prelude::Expr;
use crate::scryfall::search::query_parser::fragment::Fragment;
use crate::scryfall::search::query_parser::lexer::TokenTy;

pub struct Group<'i> {
    pub inner: Box<Union<'i>>,
}

impl<'i> Group<'i> {
    /// Consume a group of filters, started by a left parentheses.
    pub fn parse(parser: &mut Parser<'i>) -> Option<Self> {
        let lparen = parser.next_if_is(TokenTy::LParen)?.frag.clone();
        let union = Union::parse(parser);

        if parser.next_if_is(TokenTy::RParen).is_none() {
            parser.diagnostics.push(Diagnostic::Warning {
                message: "missing closing parentheses".to_string(),
                fragment: lparen,
            });
            
            None
        } else {
            Some(Self {
                inner: Box::new(union),
            })
        }
    }

    pub(super) fn consume(pair: Pair<'i, Rule>) -> Self {
        assert_eq!(
            pair.as_rule(),
            Rule::group,
            "{:?} is not a group",
            pair.as_rule()
        );
        Self {
            inner: Box::new(Union::consume(unwrap_exactly_one(pair))),
        }
    }
}

impl<'i> MapToPolarsExpr for Group<'i> {
    fn as_pexpr(&self) -> Expr {
        self.inner.as_pexpr()
    }
}
