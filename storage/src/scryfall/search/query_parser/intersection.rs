use crate::scryfall::search::polars_mapping::MapToPolarsExpr;
use crate::scryfall::search::query_parser::item::Item;
use polars::prelude::Expr;
use crate::scryfall::search::query_parser::fragment::Fragment;
use crate::scryfall::search::query_parser::Parser;
use crate::scryfall::search::query_parser::lexer::TokenTy;

#[derive(Debug)]
pub struct Intersection {
    pub items: Vec<Item>,
}

impl Intersection {
    pub fn parse(parser: &mut Parser) -> Option<Self> {
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
    
    pub fn fragment(&self) -> Fragment {
        match (self.items.first(), self.items.last()) {
            (Some(first), Some(last)) => Fragment::cover(&first.cover, &last.cover),
            _ => panic!("Intersection has no items"),
        }
    }
}

impl MapToPolarsExpr for Intersection {
    fn as_pexpr(&self) -> Expr {
        use polars::prelude::*;

        self.items
            .iter()
            .fold(lit(true), |acc, item| acc.and(item.as_pexpr()))
    }
}
