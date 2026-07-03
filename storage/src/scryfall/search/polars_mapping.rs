//! Map parsed queries to polars expressions

use std::ops::Deref;
use polars::prelude::{col, IntoLazy, LazyFrame, Expr as PExpr};
use schemas::oshibana::SearchColumn;
use crate::scryfall::ScryfallStorage;
use crate::scryfall::search::query_parser::Query;

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

pub trait MapToPolarsExpr {
    fn as_pexpr(&self) -> PExpr;
}

/// Panics
pub fn apply_filters(
    query: Query,
    storage: &ScryfallStorage,
) -> LazyFrame {
    // clone reference to cards dataframe, make it lazy
    let cards_lf = storage.cards_df.lock()
        .unwrap()
        .as_ref()
        .expect("cards_df should not be none")
        .clone()
        .lazy();

    cards_lf.filter(query.as_pexpr())
}

/// The normal card image uri will always be the last col.
pub fn apply_select(lf: LazyFrame, cols: impl Deref<Target = [SearchColumn]>) -> LazyFrame {
    let image_normal_uri_expr = col("image_uris")
        .struct_()
        .field_by_name("normal")
        .alias("_normal_image_uri");

    let mut cols = resolve_cols(cols);
    cols.push(image_normal_uri_expr);
    lf.select(cols)
}
