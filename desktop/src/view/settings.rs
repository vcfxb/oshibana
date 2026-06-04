use crate::app::Oshibana;
use crate::view::{logic_noop, ui_noop, View};
use eframe::Frame;
use egui::{DragValue, Ui};
use egui_extras::{Column, TableBody, TableBuilder};
use std::sync::atomic::Ordering;

pub const SETTINGS: View = View {
    ui: settings_ui,
    logic: logic_noop,
    menu: ui_noop,
};

fn settings_ui(app: &mut Oshibana, ui: &mut Ui, frame: &mut Frame) {
    let available_width = ui.available_width();
    TableBuilder::new(ui)
        .striped(true)
        .column(Column::initial(available_width / 3.0).resizable(true))
        .column(Column::remainder().resizable(true))
        .body(|mut body| {
            add_autosave_selection(app, &mut body);
        });
}

fn add_autosave_selection(app: &mut Oshibana, table: &mut TableBody) {
    let mut value = app.user_data_storage
        .autosave_interval_secs
        .load(Ordering::Acquire);

    let mut disable_autosave = value == f32::INFINITY;
    let original_disable_autosave = disable_autosave;

    table.row(14.0, |mut row| {
        row.col(|ui| {

            ui.label("Autosave Interval (cannot be 0)")
                .on_hover_text("Oshibana can check for changes and autosave everything for you every few seconds");
        });

        row.col(|ui| {
            ui.scope(|ui| {
                if disable_autosave {
                    ui.disable();
                }

                ui.add(DragValue::new(&mut value).speed(1.0).range(0.2..=60.0*60.0).suffix(" sec"));
            });
        });
    });


    table.row(14.0, |mut row| {
        row.col(|ui| {
            ui.label("Disable autosave");
        });

        row.col(|ui| {
            ui.checkbox(&mut disable_autosave, "");
        });
    });

    if disable_autosave {
        value = f32::INFINITY;
    }
    else if disable_autosave != original_disable_autosave {
        value = 2.0;
    }

    app.user_data_storage.autosave_interval_secs.store(value, Ordering::Release);
}
