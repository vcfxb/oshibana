use crate::scryfall::search::polars_mapping::MapToPolarsExpr;
use crate::scryfall::search::query_parser::Parser;
use crate::scryfall::search::query_parser::filter::Filter;
use crate::scryfall::search::query_parser::fragment::Fragment;
use crate::scryfall::search::query_parser::group::Group;
use crate::scryfall::search::query_parser::lexer::TokenTy;
use polars::prelude::Expr;
use std::sync::Arc;

#[derive(Debug)]
pub struct Item {
    pub cover: Fragment,
    pub modifier: Option<Modifier>,
    pub inner: ItemInner,
}

#[derive(Debug)]
pub enum ItemInner {
    Group(Group),
    Filter(Filter),
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum Modifier {
    Neg,
}

impl Item {
    pub fn parse(parser: &mut Parser) -> Option<Self> {
        let starting_byte_idx = parser.bytes_consumed();
        let modifier_token = parser.next_if_is(TokenTy::Neg).cloned();
        let modifier = modifier_token.as_ref().map(|_| Modifier::Neg);
        let Some(inner) = ItemInner::parse(parser) else {
            // parser.diagnostics.push(Diagnostic::Error {
            //     message: "failed to parse item in query".to_string(),
            //     fragment: None,
            // });

            return None;
        };

        Some(Self {
            cover: Fragment {
                full_query: Arc::clone(&parser.full_query),
                byte_range: starting_byte_idx..parser.bytes_consumed(),
            },
            modifier,
            inner,
        })
    }
}

impl MapToPolarsExpr for Item {
    fn as_pexpr(&self) -> Expr {
        use polars::prelude::*;

        match self {
            Item {
                modifier: Some(Modifier::Neg),
                inner,
                ..
            } => not(inner.as_pexpr()),
            Item {
                modifier: None,
                inner,
                ..
            } => inner.as_pexpr(),
        }
    }
}

impl ItemInner {
    fn parse(parser: &mut Parser) -> Option<Self> {
        match parser.peek()?.kind {
            TokenTy::LParen => Group::parse(parser).map(Self::Group),
            _ => Filter::parse(parser).map(Self::Filter),
        }
    }
}

impl MapToPolarsExpr for ItemInner {
    fn as_pexpr(&self) -> Expr {
        match self {
            ItemInner::Group(group) => group.as_pexpr(),
            ItemInner::Filter(filter) => filter.as_pexpr(),
        }
    }
}
