use crate::util::scryfall_symbols;
use egui::{Label, Response, TextBuffer, TextWrapMode, Ui, Widget};
use polars::prelude::AnyValue;
use schemas::oshibana::SearchColumn;
use storage::scryfall::ScryfallStorage;
use crate::util::scryfall_symbols::ResolvedPart;

/// Formatting for columns
pub fn col_format<'s>(scryfall_store: &'s ScryfallStorage, c: SearchColumn) -> Box<dyn Fn(&AnyValue, &mut Ui) -> Response + 's> {
    let str_col = |v: &AnyValue, ui: &mut Ui| {
        Label::new(v.str_value())
            .wrap_mode(TextWrapMode::Truncate)
            .ui(ui)
    };

    let mana_cost_col = |v: &AnyValue, ui: &mut Ui| {
        ui.horizontal_wrapped(|ui| {
            let s = v.str_value();
            for part in scryfall_symbols::render(scryfall_store, s.as_str()) {
                match part {
                    ResolvedPart::RenderSymbolUri(uri) => {
                        // because the egui_extras file loader seems to have issues properly
                        // handling url encoding in file names, we do this to force a misformatted
                        // uri that doesn't have escapes in it.
                        // see https://github.com/emilk/egui/issues/8288
                        match uri.to_file_path() {
                            Ok(path) => {
                                let reformatted = if cfg!(windows) {
                                    format!("file:///{}", path.display())
                                } else {
                                    format!("file://{}", path.display())
                                };

                                ui.image(reformatted);
                            },
                            Err(_) => {
                                ui.image(uri.as_str());
                            },
                        };
                        // ui.image(uri.as_str());
                    }

                    ResolvedPart::Text(text) => {
                        ui.label(text);
                    }
                }
            }
        }).response
    };

    match c {
        SearchColumn::Name | SearchColumn::Type => Box::new(str_col),
        SearchColumn::ManaCost => Box::new(mana_cost_col),
    }
}
