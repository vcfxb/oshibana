use crate::scryfall::search::polars_mapping::MapToPolarsExpr;
use crate::scryfall::search::query_parser::Rule;
use crate::scryfall::search::query_parser::filter::Filter;
use crate::scryfall::search::query_parser::group::Group;
use pest::iterators::Pair;
use polars::prelude::Expr;
use crate::scryfall::search::query_parser::fragment::Fragment;

pub struct Item<'i> {
    pub cover: Fragment<'i>,
    pub modifier: Option<Modifier>,
    pub inner: ItemInner<'i>,
}

pub enum ItemInner<'i> {
    Group(Group<'i>),
    Filter(Filter<'i>),
}

pub enum Modifier {
    Neg,
}

impl<'i> Item<'i> {
    pub(super) fn consume(pair: Pair<'i, Rule>) -> Self {
        assert_eq!(
            pair.as_rule(),
            Rule::item,
            "{:?} is not an item",
            pair.as_rule()
        );
        let mut inner = pair.into_inner();
        let first = inner.next().unwrap();
        let second = inner.next();

        match (first.as_rule(), second) {
            (Rule::modifier, Some(second)) => Self {
                modifier: Some(Modifier::consume(first)),
                inner: ItemInner::consume(second),
            },

            (_, None) => Self {
                modifier: None,
                inner: ItemInner::consume(first),
            },

            other => panic!("unrecognized item: {other:?}"),
        }
    }
}

impl<'i> MapToPolarsExpr for Item<'i> {
    fn as_pexpr(&self) -> Expr {
        use polars::prelude::*;

        match self {
            Item {
                modifier: Some(Modifier::Neg),
                inner,
            } => not(inner.as_pexpr()),
            Item {
                modifier: None,
                inner,
            } => inner.as_pexpr(),
        }
    }
}

impl<'i> ItemInner<'i> {
    fn consume(pair: Pair<'i, Rule>) -> Self {
        match pair.as_rule() {
            Rule::group => Self::Group(Group::consume(pair)),
            Rule::filter => Self::Filter(Filter::consume(pair)),
            other => panic!("illegal item rule: {other:?}"),
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

impl Modifier {
    fn consume(pair: Pair<Rule>) -> Self {
        match pair.as_str() {
            "-" => Self::Neg,
            other => panic!("`{other}` is not a valid modifier"),
        }
    }
}
