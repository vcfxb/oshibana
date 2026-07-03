use crate::app::Oshibana;
use crate::view::{View, logic_noop, ui_noop};
use eframe::Frame;
use egui::{DragValue, Layout, Ui};
use egui_extras::{Column, TableBody, TableBuilder};
use chrono::Duration;
use eframe::emath::Align;

pub const SETTINGS: View = View {
    ui: settings_ui,
    logic: logic_noop,
    menu: ui_noop,
};

fn settings_ui(app: &mut Oshibana, ui: &mut Ui, _: &mut Frame) {
    let available_width = ui.available_width();
    TableBuilder::new(ui)
        .striped(true)
        .column(Column::initial(available_width / 3.0).resizable(true))
        .column(Column::remainder().resizable(true))
        .body(|mut body| {
            add_autosave_selection(app, &mut body);
            add_scyfall_sync_option(app, &mut body);
        });
}

fn add_autosave_selection(app: &mut Oshibana, table: &mut TableBody) {
    let current_value = app
        .user_data_storage
        .loaded
        .lock()
        .unwrap().autosave_interval
        .map(Duration::as_seconds_f32)
        .unwrap_or(f32::INFINITY);

    let mut new_value = current_value;
    let mut enable_autosave = new_value != f32::INFINITY;

    table.row(14.0, |mut row| {
        row.col(|ui| {
            ui.label("Autosave Interval").on_hover_text(
                "Oshibana can check for changes and autosave everything for you every few seconds",
            );
        });

        row.col(|ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                if ui.checkbox(&mut enable_autosave, ()).changed() {
                    match enable_autosave {
                        true => { new_value = 2.0; }
                        false => { new_value = f32::INFINITY; }
                    }
                }

                if enable_autosave {
                    ui.add(
                        DragValue::new(&mut new_value)
                            .speed(1.0)
                            .range(0.2..=60.0 * 60.0)
                            .suffix(" seconds"),
                    );
                }
            });
        });
    });

    if new_value != current_value {
        let new_duration = match new_value {
            f32::INFINITY => None,
            _ => Some(Duration::milliseconds((new_value * 1000.0) as i64)),
        };

        app.user_data_storage
            .loaded
            .lock()
            .unwrap().autosave_interval = new_duration;

        app.user_data_storage.mark_pending();
    }
}

fn add_scyfall_sync_option(app: &mut Oshibana, table: &mut TableBody) {
    let current_value = app
        .user_data_storage
        .loaded
        .lock()
        .unwrap()
        .scryfall_sync_interval;

    let mut new_value = current_value.map(|d| d.num_hours());

    table.row(14.0, |mut row| {
        row.col(|ui| {
            ui.label("Scryfall autosync interval")
                .on_hover_text("\
                    If enabled, trigger a scryfall sync whenever the app is launched with \
                    scryfall data older than specified.");
        });

        row.col(|ui| {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                let mut is_checked = current_value.is_some();
                if ui.checkbox(&mut is_checked, ()).changed() {
                    match is_checked {
                        true => {
                            new_value.get_or_insert(Duration::days(1).num_hours());
                        }

                        false => {
                            new_value = None;
                        }
                    }
                }

                if let Some(value) = new_value.as_mut() {
                    ui.add(DragValue::new(value)
                        .speed(1.0)
                        .range(6..=1000)
                        .suffix(" hours"));
                }
            });
        });
    });

    if new_value != current_value.map(|d| d.num_hours()) {
        app.user_data_storage.loaded.lock().unwrap().scryfall_sync_interval = new_value.map(Duration::hours);
        app.user_data_storage.mark_pending();
    }
}
