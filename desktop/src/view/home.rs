//! Home View

use egui::{MenuBar, Panel, Ui, ViewportCommand};
use crate::app::Oshibana;
use crate::view::{logic_noop, View, MENU_BAR_ID};
use eframe::Frame;

pub const HOME: View = View {
    ui: home_ui,
    logic: logic_noop,
};

pub struct Home;

fn home_ui(app: &mut Oshibana, ui: &mut Ui, _frame: &mut Frame) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        let last_scryfall_sync = app.user_data_storage
            .loaded
            .read()
            .unwrap()
            .last_scryfall_sync
            .unwrap_or_default()
            .with_timezone(&chrono::Local)
            .to_rfc2822();

        ui.heading("Welcome to Oshibana");
        ui.label("Scryfall data is loaded and ready.");
        ui.label(format!("Last scryfall sync: {last_scryfall_sync}"))
    });
}
