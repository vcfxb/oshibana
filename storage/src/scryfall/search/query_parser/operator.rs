use crate::scryfall::search::query_parser::Rule;
use pest::iterators::Pair;

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
    pub(super) fn consume(pair: Pair<Rule>) -> Self {
        match pair.as_str() {
            "!=" => Self::Neq,
            "=" => Self::Eq,
            ":" => Self::Colon,
            "<=" => Self::Lte,
            ">=" => Self::Gte,
            "<" => Self::Lt,
            ">" => Self::Gt,
            other => panic!("`{other}` is not a valid operator"),
        }
    }
}
