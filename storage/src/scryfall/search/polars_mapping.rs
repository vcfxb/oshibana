//! Map parsed queries to polars expressions

use std::ops::Deref;
use polars::prelude::{col, IntoLazy, LazyFrame, Expr as PExpr, lit};
use schemas::oshibana::SearchColumn;
use crate::scryfall::ScryfallStorage;
use crate::scryfall::search::query_parser::Expr;

fn resolve_cols(cols: impl Deref<Target = [SearchColumn]>) -> Vec<PExpr> {
    let mut resolved = Vec::with_capacity(cols.len());

    for c in cols.iter() {
        let mapped = match c {
            SearchColumn::Name => col("name").alias("Name"),
            SearchColumn::Type => col("type_line").alias("Type"),
        };

        resolved.push(mapped);
    }

    resolved
}

/// Function assumes any/all necessary frames are joined together.
fn map_expr(e: Expr) -> PExpr {
    match e {
        Expr::Atom(atom) => {
            col("name").str().contains_literal(lit(atom))
        }

        Expr::Exact(atom) => {
            col("name").eq(lit(atom))
        }

        Expr::Negated(expr) => {
            map_expr(*expr).not()
        }

        Expr::Intersection(mut exprs) => {
            let mut root = map_expr(exprs.pop().unwrap());

            while let Some(next) = exprs.pop() {
                root = root.and(map_expr(next));
            }

            root
        }
    }
}

/// Panics
pub fn apply_filters(
    query: Expr,
    storage: &ScryfallStorage,
) -> LazyFrame {
    // clone reference to cards dataframe, make it lazy
    let cards_lf = storage.cards_df.lock()
        .unwrap()
        .as_ref()
        .expect("cards_df should not be none")
        .clone()
        .lazy();

    cards_lf.filter(map_expr(query))
}

pub fn apply_select(lf: LazyFrame, cols: impl Deref<Target = [SearchColumn]>) -> LazyFrame {
    lf.select(resolve_cols(cols))
}
