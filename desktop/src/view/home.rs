//! Home View

use crate::app::Oshibana;
use crate::view::search::SEARCH;
use crate::view::{logic_noop, ui_noop, View};
use eframe::Frame;
use egui::Ui;

pub const HOME: View = View {
    ui: home_ui,
    logic: logic_noop,
    menu: ui_noop,
};

pub struct Home;

fn home_ui(app: &mut Oshibana, ui: &mut Ui, _frame: &mut Frame) {
    let last_scryfall_sync = app
        .user_data_storage
        .loaded
        .read()
        .unwrap()
        .last_scryfall_sync
        .unwrap_or_default()
        .with_timezone(&chrono::Local)
        .format("%Y %B %e, %r");

    ui.horizontal_centered(|ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Welcome to Oshibana");
            ui.label("Scryfall data is loaded");
            ui.label(format!("Last scryfall sync: {last_scryfall_sync}"));
            ui.add_space(10.0);
            if ui.button("Search for Cards").clicked() {
                app.current_view = SEARCH;
            }
        });
    });
}
