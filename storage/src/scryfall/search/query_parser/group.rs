use crate::scryfall::search::polars_mapping::MapToPolarsExpr;
use crate::scryfall::search::query_parser::{Union, Parser, Diagnostic};
use polars::prelude::Expr;
use crate::scryfall::search::query_parser::lexer::TokenTy;

#[derive(Debug)]
pub struct Group {
    pub inner: Box<Union>,
}

impl Group {
    /// Consume a group of filters, started by a left parentheses.
    pub fn parse(parser: &mut Parser) -> Option<Self> {
        let lparen = parser.next_if_is(TokenTy::LParen)?.frag.clone();
        let union = Union::parse(parser);
        let rparen = parser.next_if_is(TokenTy::RParen);
        
        if rparen.is_none() {
            parser.diagnostics.push(Diagnostic::Warning {
                message: "missing closing parentheses".to_string(),
                fragment: lparen.clone(),
            });
        }

        Some(Self {
            inner: Box::new(union),
        })
    }
}

impl MapToPolarsExpr for Group {
    fn as_pexpr(&self) -> Expr {
        self.inner.as_pexpr()
    }
}
