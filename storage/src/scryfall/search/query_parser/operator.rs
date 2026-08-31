use crate::scryfall::search::query_parser::lexer::{Token, TokenTy};
use polars::prelude::Expr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Operator {
    Eq,
    Colon,
    Gte,
    Lte,
    Gt,
    Lt,
    Neq,
}

impl Operator {
    pub fn parse(token: &Token) -> Option<Self> {
        match token.kind {
            TokenTy::BangEq => Some(Self::Neq),
            TokenTy::Eq => Some(Self::Eq),
            TokenTy::Colon => Some(Self::Colon),
            TokenTy::LtEq => Some(Self::Lte),
            TokenTy::GtEq => Some(Self::Gte),
            TokenTy::Lt => Some(Self::Lt),
            TokenTy::Gt => Some(Self::Gt),
            _ => None,
        }
    }

    // this could be generic but there's probably no reason for it tbh.
    /// Get the [polars] [Expr] operation for the given operator.
    /// Colon becomes the same as equality.
    pub fn polars_fn(self) -> fn(Expr, Expr) -> Expr {
        match self {
            Operator::Eq | Operator::Colon => Expr::eq,
            Operator::Gte => Expr::gt_eq,
            Operator::Lte => Expr::lt_eq,
            Operator::Gt => Expr::gt,
            Operator::Lt => Expr::lt,
            Operator::Neq => Expr::neq,
        }
    }
}
