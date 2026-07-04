use egui::{Label, Response, TextWrapMode, Ui, Widget};
use polars::prelude::AnyValue;
use schemas::oshibana::SearchColumn;

/// Formatting for columns
pub fn col_format(c: SearchColumn) -> fn(&AnyValue, &mut Ui) -> Response {
    let str_col = |v: &AnyValue, ui: &mut Ui| {
        Label::new(v.str_value())
            .wrap_mode(TextWrapMode::Truncate)
            .ui(ui)
    };

    let mana_cost_col = |_v: &AnyValue, ui: &mut Ui| {
        ui.horizontal_wrapped(|_ui| {
            todo!()
        }).response
    };

    match c {
        SearchColumn::Name => str_col,
        SearchColumn::Type => str_col,
        SearchColumn::ManaCost => mana_cost_col,
    }
}
