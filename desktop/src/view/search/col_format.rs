use egui::Ui;
use polars::prelude::AnyValue;
use schemas::oshibana::SearchColumn;

/// Formatting for columns
pub fn col_format(c: SearchColumn) -> Box<dyn Fn(AnyValue, &mut Ui)> {
    let str_col = Box::new(|v: AnyValue, ui: &mut Ui| {
        ui.label(v.str_value());
    });

    match c {
        SearchColumn::Name => str_col,
        SearchColumn::Type => str_col,
    }
}