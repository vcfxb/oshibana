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
    pub fn parse(token: &Token) -> Result<Self, ()> {
        match token.kind {
            TokenTy::BangEq => Ok(Self::Neq),
            TokenTy::Eq => Ok(Self::Eq),
            TokenTy::Colon => Ok(Self::Colon),
            TokenTy::LtEq => Ok(Self::Lte),
            TokenTy::GtEq => Ok(Self::Gte),
            TokenTy::Lt => Ok(Self::Lt),
            TokenTy::Gt => Ok(Self::Gt),
            _ => Err(()),
        }
    }
}
