use crate::storage::scryfall::{ScryfallStorage, pull_handler::SyncState};
use crate::views::View;
use clients::scryfall::ScryfallClient;
use eframe::Frame;
use egui::{
    Context, IconData, Key, KeyboardShortcut, Modifiers, Panel, ViewportCommand,
    containers::menu::MenuBar,
};
use humansize::{FormatSizeOptions, Kilo, format_size, format_size_i};
use std::sync::Arc;
use std::sync::atomic::Ordering;

pub struct Oshibana {
    scryfall_storage: ScryfallStorage,
    #[expect(dead_code)]
    icon: Arc<IconData>,
    #[expect(dead_code)]
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

                let state = *self.scryfall_storage.pull_handler.sync_state.lock().unwrap();

                let total = match state {
                    SyncState::Downloading => {
                        self.scryfall_storage
                            .pull_handler
                            .sync_size
                            .load(Ordering::Relaxed)
                    }

                    // handle fswrite and idle here as well ig
                    SyncState::Transforming | SyncState::FsWrite | SyncState::Idle => {
                        self.scryfall_storage
                            .pull_handler
                            .total_cards
                            .load(Ordering::Relaxed)
                    }
                };

                let progress = match state {
                    SyncState::Downloading => {
                        self.scryfall_storage
                            .pull_handler
                            .displayed_bytes
                            .load(Ordering::Relaxed)
                    }

                    SyncState::Transforming | SyncState::FsWrite | SyncState::Idle => {
                        self.scryfall_storage
                            .pull_handler
                            .displayed_cards_transformed
                            .load(Ordering::Relaxed)
                    }
                };

                let progress_percent = progress as f32 / total.max(1) as f32;

                let progress_bar = egui::ProgressBar::new(progress_percent)
                    .show_percentage()
                    .desired_width(ui.available_width() - 40.0);

                ui.add(progress_bar);

                match state {
                    SyncState::Downloading => {
                        let rate = self.scryfall_storage
                            .pull_handler
                            .displayed_rate
                            .load(Ordering::Relaxed);

                        let format_options = FormatSizeOptions::default()
                            .decimal_places(2)
                            .kilo(Kilo::Decimal);

                        let rate_text = format_size_i(rate, format_options);

                        ui.label(format!(
                            "{}/{} ({}/s)",
                            format_size(progress, format_options),
                            format_size(total, format_options),
                            rate_text
                        ));
                    }

                    SyncState::Transforming => {
                        ui.label(format!(
                            "{progress}/{total}",
                        ));
                    }

                    _ => {}
                }

                match state {
                    SyncState::Downloading => ui.label("Downloading & Deserializing"),
                    SyncState::Transforming => ui.label("Reformatting Card Data"),
                    SyncState::FsWrite => ui.label("Writing card data file"),
                    SyncState::Idle => ui.label("Done!"),
                };

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

        let current_sync_state = *self
            .scryfall_storage
            .pull_handler
            .sync_state
            .lock()
            .unwrap();
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
