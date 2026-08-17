use crate::scryfall::search::polars_mapping::MapToPolarsExpr;
use crate::scryfall::search::query_parser::item::Item;
use polars::prelude::Expr;
use crate::scryfall::search::query_parser::fragment::Fragment;
use crate::scryfall::search::query_parser::Parser;
use crate::scryfall::search::query_parser::lexer::TokenTy;

#[derive(Debug)]
pub struct Intersection<'i> {
    pub items: Vec<Item<'i>>,
}

impl<'i> Intersection<'i> {
    pub fn parse(parser: &mut Parser<'i>) -> Option<Self> {
        let mut items = Vec::new();
        
        while let Some(item) = Item::parse(parser) {
            items.push(item);

            if parser.next_if_is(TokenTy::And).is_some() {
                continue;
            }

            // If there's an 'And', consume it. If there's an 'Or', we stop because that breaks the intersection.
            match parser.peek().map(|t| t.kind) {
                Some(TokenTy::Or | TokenTy::RParen) => break,
                _ => continue,
            }
        }
        
        if items.is_empty() {
            None
        } else {
            Some(Self { items })
        }
    }
    
    pub fn fragment(&self) -> Fragment<'i> {
        match (self.items.first(), self.items.last()) {
            (Some(first), Some(last)) => Fragment::cover(&first.cover, &last.cover),
            _ => panic!("Intersection has no items"),
        }
    }
}

impl<'i> MapToPolarsExpr for Intersection<'i> {
    fn as_pexpr(&self) -> Expr {
        use polars::prelude::*;

        self.items
            .iter()
            .fold(lit(true), |acc, item| acc.and(item.as_pexpr()))
    }
}
