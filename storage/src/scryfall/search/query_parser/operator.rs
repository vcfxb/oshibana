use crate::scryfall::search::query_parser::lexer::{Token, TokenTy};

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
}
