use crate::util::scryfall_symbols;
use crate::util::scryfall_symbols::ResolvedPart;
use egui::{Label, Response, TextBuffer, TextWrapMode, Ui, Widget};
use polars::prelude::AnyValue;
use schemas::oshibana::SearchColumn;
use storage::scryfall::ScryfallStorage;

pub type ValueFormatter<'s> = Box<dyn Fn(&AnyValue, &mut Ui) -> Response + 's>;

pub fn make_text_formatter() -> ValueFormatter<'static> {
    Box::new(|v: &AnyValue, ui: &mut Ui| {
        Label::new(v.str_value())
            .wrap_mode(TextWrapMode::Truncate)
            .ui(ui)
    })
}


pub fn make_symbol_rendering_formatter(store: &ScryfallStorage) -> ValueFormatter<'_> {
    Box::new(|v: &AnyValue, ui: &mut Ui| {
        ui.horizontal_wrapped(|ui| {
            let s = v.str_value();
            for part in scryfall_symbols::render(store, s.as_str()) {
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
                            }
                            Err(_) => {
                                ui.image(uri.as_str());
                            }
                        };
                        // ui.image(uri.as_str());
                    }

                    ResolvedPart::Text(text) => {
                        ui.label(text);
                    }
                }
            }
        }).response
    })
}

pub fn col_format(s: &ScryfallStorage, c: SearchColumn) -> ValueFormatter<'_> {
    match c {
        SearchColumn::Name | SearchColumn::Type => make_text_formatter(),
        SearchColumn::ManaCost => make_symbol_rendering_formatter(s),
    }
}
