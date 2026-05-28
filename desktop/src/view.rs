//! Different views that the application uses,

use egui::{Context, MenuBar, Panel, Ui, ViewportCommand};
use eframe::Frame;
use crate::app::Oshibana;

pub mod home;

#[derive(Copy, Clone)]
pub struct View {
    pub ui: fn(&mut Oshibana, &mut Ui, &mut Frame),
    pub logic: fn(&mut Oshibana, &Context, &mut Frame),
}

const fn ui_noop(_: &mut Oshibana, _: &mut Ui, _: &mut Frame) {}
const fn logic_noop(_: &mut Oshibana, _: &Context, _: &mut Frame) {}

fn paint_top_bar(app: &Oshibana, ui: &mut Ui) {
    Panel::top("menu_bar_panel").show_inside(ui, |ui| {
        MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Pull latest scryfall data").clicked() {
                    app.scryfall_storage.trigger_sync();
                }

                ui.separator();

                if ui.button("Quit").clicked() {
                    ui.send_viewport_cmd(ViewportCommand::Close);
                }
            });
        })
    });
}
