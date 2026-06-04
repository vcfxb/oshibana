//! Card Search view

use crate::app::Oshibana;
use crate::view::{logic_noop, View};
use eframe::Frame;
use egui::{FontFamily, FontId, TextEdit, Ui};

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
        });
    });
}

fn search_ui(app: &mut Oshibana, ui: &mut Ui, _: &mut Frame) {
    ui.horizontal_top(|ui| ui.vertical_centered(|ui| {
        let search_bar = TextEdit::singleline(&mut app.search_state.search_text)
            .desired_width(ui.available_width() - 40.0)
            .font(FontId { size: 16.0, family: FontFamily::Monospace });
        ui.add(search_bar);
        ui.separator();
    }));


}

