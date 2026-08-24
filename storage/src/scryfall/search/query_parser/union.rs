use crate::scryfall::search::polars_mapping::MapToPolarsExpr;
use crate::scryfall::search::query_parser::intersection::Intersection;
use polars::prelude::Expr;
use crate::scryfall::search::query_parser::fragment::Fragment;
use crate::scryfall::search::query_parser::Parser;
use crate::scryfall::search::query_parser::lexer::TokenTy;

#[derive(Debug)]
pub struct Union {
    pub intersections: Vec<Intersection>,
}

impl Union {
    pub fn parse(parser: &mut Parser) -> Self {
        let mut intersections = Vec::new();
        
        while let Some(intersection) = Intersection::parse(parser) {
            intersections.push(intersection);
            
            if let Some(token) = parser.peek() {
                if token.kind == TokenTy::Or {
                    parser.pull(); // consume Or
                // } else if token.kind == TokenTy::RParen {
                //     // end of group
                //     break;
                } else {
                    // unexpected token? Or is this another union somehow?
                    // intersection should consume until it hits 'or' or ')'
                    break; 
                }
            } else {
                break;
            }
        }
        
        // if intersections.is_empty() {
        //     None
        // } else {
        //     Some(Self { intersections })
        // }
        
        Self { intersections }
    }
    
    pub fn conver_fragment(&self) -> Fragment {
        match (self.intersections.first(), self.intersections.last()) {
            (Some(first), Some(last)) => Fragment::cover(&first.fragment(), &last.fragment()),
            _ => panic!("Union has no intersections"),
        }
    }
}

impl MapToPolarsExpr for Union {
    fn as_pexpr(&self) -> Expr {
        use polars::prelude::*;

        self.intersections
            .iter()
            .fold(lit(false), |acc, item| acc.or(item.as_pexpr()))
    }
}
