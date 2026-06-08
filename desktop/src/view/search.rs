//! Card Search view

pub mod col_format;

use crate::app::Oshibana;
use crate::view::{logic_noop, View};
use eframe::Frame;
use egui::{FontFamily, FontId, ScrollArea, TextEdit, Ui};
use egui_extras::{Column, TableBuilder};
use polars::prelude::AnyValue;
use schemas::oshibana::SearchColumn;
use strum::IntoEnumIterator;

pub const SEARCH: View = View {
    ui: search_ui,
    logic: logic_noop,
    menu: search_menu,
};

#[derive(Debug, Default)]
pub enum SearchLayout {
    #[default]
    Rows,
    Tiles,

    /// Card images next to their info/oracle_text/etc
    Detailed,
}

#[derive(Debug, Default)]
pub struct SearchState {
    pub search_text: String,
    pub layout: SearchLayout,
}

fn search_menu(app: &mut Oshibana, ui: &mut Ui, _: &mut Frame) {
    ui.menu_button("Search", |ui| {
        ui.menu_button("Layout", |ui| {
            if ui.button("Rows").clicked() {
                app.search_state.layout = SearchLayout::Rows;
            }

            if ui.button("Tiles").clicked() {
                app.search_state.layout = SearchLayout::Tiles;
            }

            if ui.button("Detailed").clicked() {
                app.search_state.layout = SearchLayout::Detailed;
            }
        });

        let mut user_data_lock = app.user_data_storage.loaded.lock().unwrap();

        ui.menu_button("Columns", |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                for col in SearchColumn::iter() {
                    let selected = user_data_lock.visible_search_columns.contains(&col);
                    if ui.selectable_label(selected, col.into_str()).clicked() {
                        if !selected {
                            user_data_lock.visible_search_columns.push(col);
                        } else {
                            user_data_lock.visible_search_columns.retain(|s| s != &col);
                        }
                        app.user_data_storage.mark_pending();
                    }
                }
            })
        })
    });
}

fn search_ui(app: &mut Oshibana, ui: &mut Ui, _: &mut Frame) {
    ui.horizontal_top(|ui| {
        ui.vertical_centered(|ui| {
            let search_bar = TextEdit::singleline(&mut app.search_state.search_text)
                .desired_width(ui.available_width() - 40.0)
                .font(FontId {
                    size: 16.0,
                    family: FontFamily::Monospace,
                });
            ui.add(search_bar);
            ui.separator();
        })
    });

    let user_data_guard = app.user_data_storage.loaded.lock().unwrap();
    let visible_cols = &user_data_guard.visible_search_columns;
    let formatting_fns = visible_cols.iter()
        .map(|col| col_format::col_format(*col))
        .collect::<Vec<Box<dyn Fn(AnyValue, &mut Ui)>>>();

    let query = app.search_state.search_text.as_str();
    let search_result = app.scryfall_storage
        .search(query, user_data_guard.visible_search_columns.as_slice());

    drop(user_data_guard);

    if let Err(err) = search_result {
        ui.label(format!("search error: {err}"));
        return;
    };

    let df = search_result.unwrap();

    TableBuilder::new(ui)
        .columns(Column::auto(), df.width())
        .resizable(true)
        .striped(true)
        .header(16.0, |mut row| {
            for search_col in df.get_column_names() {
                row.col(|ui| {
                    ui.label(search_col.as_str());
                });
            }
        })
        .body(|body| {
            body.rows(12.0, df.height(), |mut row| {
                // this is potentially slow (polars discourages row indexing),
                // but I don't know of a better way -- table's don't support adding col by col.
                let df_row = df.get_row(row.index())
                    .expect("dataframe index is in bounds");

                for (col_idx, col) in df_row.0.into_iter().enumerate() {
                    row.col(|ui| {
                        (formatting_fns[col_idx])(col, ui);
                    });
                }
            })
        });
}
