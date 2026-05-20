use crate::views::View;
use clients::scryfall::ScryfallClient;
use eframe::Frame;
use egui::{Context, IconData, Key, KeyboardShortcut, Modifiers, Panel, ViewportCommand, containers::menu::MenuBar};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use humansize::{format_size, format_size_i, FormatSizeOptions, Kilo};
use crate::storage::scryfall::{ScryfallStorage, SyncState};

pub struct Oshibana {
    scryfall_storage: ScryfallStorage,
    icon: Arc<IconData>,
    current_view: View,
    last_sync_state: SyncState,
}

impl Oshibana {
    pub fn new(_: &eframe::CreationContext<'_>, icon: Arc<IconData>) -> anyhow::Result<Self> {
        let scryfall_client = ScryfallClient::new();
        let scryfall_storage = ScryfallStorage::new(scryfall_client);
        
        if !scryfall_storage.is_ready() {
            log::warn!("scryfall storage not ready, triggering initial sync");
            scryfall_storage.trigger_sync();
        }

        log::info!("constructing application state");

        Ok(Self {
            scryfall_storage,
            icon,
            current_view: View::Home,
            last_sync_state: SyncState::Idle,
        })
    }

    fn show_loading_screen(&self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 3.0);
                ui.heading("Syncing Scryfall Data");
                ui.add_space(20.0);

                let total = self.scryfall_storage.sync_size.load(Ordering::Relaxed);
                let downloaded = self.scryfall_storage.displayed_downloaded.load(Ordering::Relaxed);
                let progress = downloaded as f32 / total.max(1) as f32;

                let width = ui.available_width();

                let progress_bar = egui::ProgressBar::new(progress)
                    .show_percentage()
                    .desired_width(ui.available_width() - 40.0);

                ui.add(progress_bar);

                let format_options = FormatSizeOptions::default()
                    .decimal_places(2)
                    .kilo(Kilo::Decimal);

                let rate = self.scryfall_storage.displayed_rate.load(Ordering::Relaxed);
                let rate_text = format_size_i(rate, &format_options);

                ui.label(format!(
                    "{}/{} ({}/s)",
                    format_size(downloaded, &format_options),
                    format_size(total, &format_options),
                    rate_text
                ));

                let state = self.scryfall_storage.sync_state.lock().unwrap().clone();
                if state == SyncState::FsWrite {
                    ui.label("Processing data (this may take a minute)");
                } else if state == SyncState::Downloading {
                    ui.label("Downloading");
                }

                ui.add_space(10.0);
                ui.spinner();
            });
        });
    }
}

impl eframe::App for Oshibana {
    fn logic(&mut self, ctx: &Context, _frame: &mut Frame) {
        static QUIT_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Q);

        let should_quit = ctx.input_mut(|input_state| input_state.consume_shortcut(&QUIT_SHORTCUT));
        if should_quit {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }

        let current_sync_state = *self.scryfall_storage.sync_state.lock().unwrap();
        if self.last_sync_state != SyncState::Idle && current_sync_state == SyncState::Idle {
            // Sync just finished, try to load data
            self.scryfall_storage.try_reload();
        }
        self.last_sync_state = current_sync_state;
        
        if current_sync_state != SyncState::Idle {
            ctx.request_repaint();
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut Frame) {
        if self.scryfall_storage.is_syncing() || !self.scryfall_storage.is_ready() {
            self.show_loading_screen(ui);
            return;
        }

        Panel::top("menu_bar_panel").show_inside(ui, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Pull latest scryfall data").clicked() {
                        self.scryfall_storage.trigger_sync();
                    }

                    ui.separator();

                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(ViewportCommand::Close);
                    }
                });
            })
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Welcome to Oshibana");
            ui.label("Scryfall data is loaded and ready.");
        });
    }
}
