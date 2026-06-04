use crate::storage::scryfall::ScryfallStorage;
use crate::view::{View, MENU_BAR_ID};
use clients::scryfall::ScryfallClient;
use eframe::Frame;
use egui::{Context, IconData, Key, KeyboardShortcut, Modifiers, Panel, ViewportCommand, containers::menu::MenuBar, RichText, Color32, Label, frame, Margin, Layout, TextFormat, Style, FontSelection, CentralPanel};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use chrono::{DateTime, Local};
use eframe::emath::Align;
use egui::text::LayoutJob;
use egui_material_icons::{icons, MaterialIcon};
use crate::storage::user_data::UserDataStorage;
use crate::view::home::HOME;
use crate::view::settings::SETTINGS;

pub struct Oshibana {
    pub scryfall_storage: ScryfallStorage,
    pub user_data_storage: Arc<UserDataStorage>,
    pub icon: Arc<IconData>,
    pub current_view: View,
}

impl Oshibana {
    pub fn new(_: &eframe::CreationContext<'_>, icon: Arc<IconData>) -> anyhow::Result<Self> {
        let scryfall_client = ScryfallClient::new();
        let scryfall_storage = ScryfallStorage::new(scryfall_client);
        let user_data = UserDataStorage::new()?;

        if !scryfall_storage.is_ready() {
            log::warn!("scryfall storage not ready, triggering initial sync");
            scryfall_storage.trigger_sync(Arc::clone(&user_data));
        }

        log::info!("constructing application state");

        Ok(Self {
            scryfall_storage,
            user_data_storage: user_data,
            icon,
            current_view: crate::view::home::HOME,
        })
    }
}

impl eframe::App for Oshibana {
    fn logic(&mut self, ctx: &Context, frame: &mut Frame) {
        // static QUIT_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Q);
        static SAVE_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::S);

        // let should_quit = ctx.input_mut(|input| input.consume_shortcut(&QUIT_SHORTCUT));
        let should_save = ctx.input_mut(|input| input.consume_shortcut(&SAVE_SHORTCUT));

        // if should_quit {
        //     ctx.send_viewport_cmd(ViewportCommand::Close);
        // }

        if should_save {
            self.user_data_storage.trigger_save();
        }

        if ctx.input(|i| i.viewport().close_requested()) {
            self.scryfall_storage.sync_handler.cancel_requested.store(true, Ordering::Relaxed);
            self.user_data_storage.save().expect("saved successfully");
        }
        
        if !self.scryfall_storage.is_ready() && !self.scryfall_storage.try_reload() {
            self.scryfall_storage.trigger_sync(Arc::clone(&self.user_data_storage));
        }

        if ctx.input(|i| i.viewport().visible()).unwrap_or(true) {
            ctx.request_repaint();
        }

        (self.current_view.logic)(self, ctx, frame);
    }

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut Frame) {
        if self.scryfall_storage.sync_handler.is_syncing() {
            self.scryfall_storage.sync_handler.ui(ui);
            return;
        }

        Panel::top("top_panel").show_inside(ui, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Home").clicked() {
                        self.current_view = HOME;
                    }

                    if ui.button("Settings...").clicked() {
                        self.current_view = SETTINGS;
                    };

                    ui.separator();

                    if ui.button("Pull latest scryfall data").clicked() {
                        self.scryfall_storage.trigger_sync(Arc::clone(&self.user_data_storage));
                    }

                    ui.separator();
                    if ui.button("Save").clicked() {
                        self.user_data_storage.trigger_save();
                    }

                    ui.separator();
                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(ViewportCommand::Close);
                    }
                });

                (self.current_view.menu)(self, ui, frame);

                ui.menu_button("Window", |ui| {
                    if ui.button("Toggle Fullscreen").clicked() {
                        let fullscreen_state = ui.input(|input| {
                            input.viewport().maximized
                        });

                        if let Some(state) = fullscreen_state {
                            ui.send_viewport_cmd(ViewportCommand::Maximized(!state));
                            ui.send_viewport_cmd(ViewportCommand::Decorations(state));
                        }
                    }
                });
            })
        });

        let has_pending = self.user_data_storage.has_pending_updates();
        let is_saving = self.user_data_storage.currently_saving();
        let base_fill = ui.visuals().panel_fill;

        if !has_pending && !is_saving {
            let translucent_green = Color32::GREEN.gamma_multiply(0.05);
            ui.visuals_mut().panel_fill = translucent_green + base_fill;
        }

        Panel::bottom("bottom_panel").show_inside(ui, |ui| {
            ui.add_space(1.0);
            ui.horizontal(|ui| {
                ui.label(Local::now().format("%Y %B %e %r").to_string());

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    match is_saving || has_pending {
                        true => {
                            ui.spinner();
                            ui.label("Saving");
                        }
                        false => {
                            ui.label(icons::ICON_CHECK.rich_text());
                            ui.label("Saved");
                        }
                    }
                });
            });
            ui.add_space(1.0);
        });

        // reset panel fill
        ui.visuals_mut().panel_fill = base_fill;

        // central panel must always go last
        CentralPanel::default().show_inside(ui, |ui| {
            (self.current_view.ui)(self, ui, frame);
        });
    }
}
