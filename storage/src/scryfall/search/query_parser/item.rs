use crate::scryfall::search::polars_mapping::MapToPolarsExpr;
use crate::scryfall::search::query_parser::filter::Filter;
use crate::scryfall::search::query_parser::fragment::Fragment;
use crate::scryfall::search::query_parser::group::Group;
use crate::scryfall::search::query_parser::lexer::TokenTy;
use crate::scryfall::search::query_parser::Parser;
use polars::prelude::Expr;

pub struct Item<'i> {
    pub cover: Fragment<'i>,
    pub modifier: Option<Modifier>,
    pub inner: ItemInner<'i>,
}

pub enum ItemInner<'i> {
    Group(Group<'i>),
    Filter(Filter<'i>),
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum Modifier {
    Neg,
}

impl<'i> Item<'i> {
    pub fn parse(parser: &mut Parser<'i>) -> Option<Self> {
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
                full_query: parser.full_query,
                byte_range: starting_byte_idx..parser.bytes_consumed(),
            },
            modifier,
            inner,
        })
    }
}

impl<'i> MapToPolarsExpr for Item<'i> {
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

impl<'i> ItemInner<'i> {
    fn parse(parser: &mut Parser<'i>) -> Option<Self> {
        match parser.peek()?.kind {
            TokenTy::LParen => Group::parse(parser).map(Self::Group),
            _ => Filter::parse(parser).map(Self::Filter),
        }
    }
}

impl<'i> MapToPolarsExpr for ItemInner<'i> {
    fn as_pexpr(&self) -> Expr {
        match self {
            ItemInner::Group(group) => group.as_pexpr(),
            ItemInner::Filter(filter) => filter.as_pexpr(),
        }
    }
}
