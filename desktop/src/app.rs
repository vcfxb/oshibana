use crate::storage::scryfall::ScryfallStorage;
use crate::view::View;
use clients::scryfall::ScryfallClient;
use eframe::Frame;
use egui::{
    Context, IconData, Key, KeyboardShortcut, Modifiers, Panel, ViewportCommand,
    containers::menu::MenuBar,
};
use std::sync::Arc;
use std::sync::atomic::Ordering;

pub struct Oshibana {
    pub scryfall_storage: ScryfallStorage,
    pub icon: Arc<IconData>,
    pub current_view: View,
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
            current_view: crate::view::home::HOME,
        })
    }
}

impl eframe::App for Oshibana {
    fn logic(&mut self, ctx: &Context, frame: &mut Frame) {
        static QUIT_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Q);

        let should_quit = ctx.input_mut(|input_state| input_state.consume_shortcut(&QUIT_SHORTCUT));
        if should_quit {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }

        if ctx.input(|i| i.viewport().close_requested()) {
            self.scryfall_storage.sync_handler.cancel_requested.store(true, Ordering::Relaxed);
        }
        
        if !self.scryfall_storage.is_ready() && !self.scryfall_storage.try_reload() {
            self.scryfall_storage.trigger_sync();
        }

        if self.scryfall_storage.sync_handler.is_syncing() {
            ctx.request_repaint();
        }
        
        (self.current_view.logic)(self, ctx, frame);
    }

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut Frame) {
        if self.scryfall_storage.sync_handler.is_syncing() {
            self.scryfall_storage.sync_handler.ui(ui);
            return;
        }

        (self.current_view.ui)(self, ui, frame);
    }
}
