use crate::storage::scryfall::ScryfallStorage;
use crate::views::View;
use clients::scryfall::ScryfallClient;
use eframe::Frame;
use egui::{
    Context, IconData, Key, KeyboardShortcut, Modifiers, Panel, ViewportCommand,
    containers::menu::MenuBar,
};
use std::sync::Arc;

pub struct Oshibana {
    scryfall_storage: ScryfallStorage,
    #[expect(dead_code)]
    icon: Arc<IconData>,
    #[expect(dead_code)]
    current_view: View,
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
        })
    }
}

impl eframe::App for Oshibana {
    fn logic(&mut self, ctx: &Context, _frame: &mut Frame) {
        static QUIT_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Q);

        let should_quit = ctx.input_mut(|input_state| input_state.consume_shortcut(&QUIT_SHORTCUT));
        if should_quit {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
        
        if !self.scryfall_storage.is_ready() && !self.scryfall_storage.try_reload() {
            self.scryfall_storage.trigger_sync();
        }

        if self.scryfall_storage.sync_handler.is_syncing() {
            ctx.request_repaint();
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut Frame) {
        if self.scryfall_storage.sync_handler.is_syncing() {
            self.scryfall_storage.sync_handler.ui(ui);
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
