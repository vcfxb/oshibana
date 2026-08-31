//! Map parsed queries to polars expressions

use crate::scryfall::search::query_parser::union::Union;
use polars::prelude::{
    Expr as PExpr, LazyFrame, Selector, SortMultipleOptions, UniqueKeepStrategy, col, format_str,
    lit, when,
};
use schemas::oshibana::{Direction, SearchViewColumn, SortBy, UniqueBy};
use std::ops::Deref;
use std::sync::Arc;

fn resolve_cols(cols: impl Deref<Target = [SearchViewColumn]>) -> Vec<PExpr> {
    let mut resolved = Vec::with_capacity(cols.len());

    for c in cols.iter() {
        let mapped = match c {
            SearchViewColumn::Name => when(col("printed_name").is_null())
                .then(col("name"))
                .otherwise(
                    format_str("{} ({})", &[col("printed_name"), col("name")])
                        .expect("name formatting"),
                )
                .alias("Name"),

            SearchViewColumn::Type => when(col("printed_type_line").is_null())
                .then(col("type_line"))
                .otherwise(
                    format_str("{} ({})", &[col("printed_type_line"), col("type_line")])
                        .expect("type line formatting"),
                )
                .alias("Type"),

            // mana cost would normally potentially need re-aliasing here, but it's done pre-search
            // to make looking up DFCs more reliable instead.
            SearchViewColumn::ManaCost => col("mana_cost").alias("Mana Cost"),
        };

        resolved.push(mapped);
    }

    resolved
}

pub trait MapToPolarsExpr {
    fn as_pexpr(&self) -> PExpr;
}

pub fn apply_grouping(cards_lf: LazyFrame, unique_by: UniqueBy) -> LazyFrame {
    match unique_by {
        UniqueBy::Cards => cards_lf
            .sort(
                ["released_at", "lang"],
                SortMultipleOptions::new()
                    .with_order_descending_multi([true, false])
                    .with_nulls_last(true)
                    .with_maintain_order(true),
            )
            .unique_stable(
                Some(Selector::ByName {
                    names: Arc::new(["oracle_id".into()]),
                    strict: true,
                }),
                UniqueKeepStrategy::First,
            ),

        UniqueBy::Printings => cards_lf,
    }
}

pub fn apply_filters(cards_lf: LazyFrame, query: &Union) -> LazyFrame {
    cards_lf.filter(query.as_pexpr())
}

/// The normal card image uri will always be the last col.
pub fn apply_select(lf: LazyFrame, cols: impl Deref<Target = [SearchViewColumn]>) -> LazyFrame {
    let image_normal_uri_expr = col("image_uris")
        .struct_()
        .field_by_name("normal")
        .alias("_normal_image_uri");

    let mut cols = resolve_cols(cols);
    cols.push(image_normal_uri_expr);
    lf.select(cols)
}

pub fn apply_preprocessing(cards_lf: LazyFrame) -> LazyFrame {
    use polars::prelude::format_str;

    cards_lf.with_columns([col("mana_cost").fill_null(
        format_str(
            "{} // {}",
            &[
                col("card_faces")
                    .list()
                    .first()
                    .struct_()
                    .field_by_name("mana_cost"),
                col("card_faces")
                    .list()
                    .get(lit(1), true)
                    .struct_()
                    .field_by_name("mana_cost"),
            ],
        )
        .expect("formatting is correct"),
    )])
}

pub fn apply_sort(cards_lf: LazyFrame, sort_by: SortBy) -> LazyFrame {
    match sort_by {
        SortBy::Name(dir) => cards_lf.sort(
            ["name"],
            SortMultipleOptions::new()
                .with_order_descending(dir == Direction::Descending)
                .with_maintain_order(true),
        ),

        SortBy::ReleaseDate(dir) => cards_lf.sort(
            ["released_at"],
            SortMultipleOptions::new()
                .with_order_descending(dir == Direction::Descending)
                .with_maintain_order(true),
        ),
    }
}
