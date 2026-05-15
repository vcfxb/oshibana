pub mod autosave;
pub mod save;
pub mod scryfall_pull;

use crate::views::View;
use clients::scryfall::ScryfallClient;
use eframe::Frame;
use egui::{Context, IconData, Key, KeyboardShortcut, Modifiers, Panel, ViewportCommand, containers::menu::MenuBar};
use std::sync::Arc;
use std::sync::atomic::Ordering;
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

                let total = self.scryfall_storage.sync_size.load(Ordering::SeqCst);
                let (download, rate) = {
                    let checkpoints = self.scryfall_storage.sync_checkpoints.lock().unwrap();
                    let oldest = checkpoints.get(0);
                    let last = checkpoints.get(checkpoints.len() - 1);

                    if let (Some((start, initial)), Some((_, r#final))) = (oldest, last) {
                        let delta = start.elapsed();
                        let bytes = r#final - initial;
                        if bytes == 0 {
                            (*r#final, 0.0)
                        } else {
                            (*r#final, bytes as f64 / delta.as_secs_f64())
                        }
                    } else {
                        (0, 0.0)
                    }
                };

                let progress = if total > 0 {
                    download as f32 / total as f32
                } else {
                    0.0
                };

                let state = *self.scryfall_storage.sync_state.lock().unwrap();
                let text = match state {
                    SyncState::Downloading => {
                         format!("Downloading {:.2} MB / {:.2} MB", download as f32 / 1_000_000.0, total as f32 / 1_000_000.0)
                    }
                    SyncState::FsWrite => "Processing data (this may take a minute)...".to_string(),
                    _ => "Waiting...".to_string(),
                };

                ui.add(egui::ProgressBar::new(progress).text(text));
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
                    if ui.button("Sync latest scryfall data").clicked() {
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
            ui.separator();
            ui.label("Scryfall data is loaded and ready.");
        });
    }
}
