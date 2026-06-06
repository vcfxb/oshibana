//! Card Search view

pub mod columns;

use crate::app::Oshibana;
use crate::view::{View, logic_noop};
use eframe::Frame;
use egui::{FontFamily, FontId, ScrollArea, TextEdit, Ui};
use egui_extras::{Column, TableBuilder};
use polars::prelude::DataFrame;
use std::sync::{Arc, Mutex};

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
    pub search_result: Arc<Mutex<DataFrame>>,
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
        });

        let mut user_data_lock = app.user_data_storage.loaded.lock().unwrap();

        ui.menu_button("Columns", |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                for name in columns::get_possible_columns() {
                    let selected = user_data_lock.visible_search_columns.contains(&name);
                    if ui.selectable_label(selected, name.as_str()).clicked() {
                        if !selected {
                            user_data_lock.visible_search_columns.push(name);
                        } else {
                            user_data_lock.visible_search_columns.retain(|s| s != &name);
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

    TableBuilder::new(ui)
        .columns(Column::auto(), visible_cols.len())
        .resizable(true)
        .striped(true)
        .header(18.0, |mut row| {
            for name in visible_cols {
                row.col(|ui| {
                    ui.label(name);
                });
            }
        })
        .body(|_body| {
            // app.scryfall_storage
            // body.rows(18.0, )
        });
}
