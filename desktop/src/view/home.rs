//! Home View

use egui::{MenuBar, Panel, Ui, ViewportCommand};
use crate::app::Oshibana;
use crate::view::{logic_noop, View};
use eframe::Frame;

pub const HOME: View = View {
    ui: home_ui,
    logic: logic_noop,
};

pub struct Home;

fn home_ui(app: &mut Oshibana, ui: &mut Ui, _frame: &mut Frame) {
    super::paint_top_bar(app, ui);

    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.heading("Welcome to Oshibana");
        ui.label("Scryfall data is loaded and ready.");
    });
}
