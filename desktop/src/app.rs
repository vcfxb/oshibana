use crate::view::View;
use crate::view::home::HOME;
use crate::view::scryfall_sync::SyncView;
use crate::view::search::{SEARCH, SearchState};
use crate::view::settings::SETTINGS;
use chrono::{Local, Utc};
use clients::scryfall::ScryfallClient;
use eframe::Frame;
use eframe::emath::Align;
use egui::{
    CentralPanel, Color32, Context, IconData, Key, KeyboardShortcut, Layout, Modifiers, Panel, Ui,
    ViewportBuilder, ViewportCommand, ViewportId, containers::menu::MenuBar,
};
use egui_material_icons::icons;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use storage::scryfall::ScryfallStorage;
use storage::user_data::UserDataStorage;

pub struct Oshibana {
    pub scryfall_storage: Arc<ScryfallStorage>,
    pub user_data_storage: Arc<UserDataStorage>,
    pub icon: Arc<IconData>,
    pub current_view: View,
    pub search_state: SearchState,
    pub sync_view_state: Arc<SyncView>,
    view_about: Arc<AtomicBool>,
}

impl Oshibana {
    pub fn new(_: &eframe::CreationContext<'_>, icon: Arc<IconData>) -> anyhow::Result<Self> {
        let scryfall_client = ScryfallClient::new();
        let scryfall_storage = Arc::new(ScryfallStorage::new(scryfall_client));
        let user_data = UserDataStorage::new()?;
        let sync_view_state = Arc::new(SyncView::new());

        let scryfall_sync_expired = {
            let ud = user_data.loaded.lock().unwrap();
            match (ud.last_scryfall_sync, ud.scryfall_sync_interval) {
                (None, _) | (_, None) => false,
                (Some(last_sync), Some(interval)) => Utc::now() - last_sync > interval,
            }
        };

        if !scryfall_storage.is_ready() {
            log::warn!("scryfall storage not ready, triggering initial sync");
        }

        if scryfall_sync_expired {
            log::info!("scryfall sync expired, triggering sync");
        }

        if !scryfall_storage.is_ready() || scryfall_sync_expired {
            let progress_cb = sync_view_state
                .clone()
                .make_progress_cb(scryfall_storage.sync_handler.clone());

            scryfall_storage
                .clone()
                .trigger_sync(Arc::clone(&user_data), progress_cb);
        }

        log::info!("constructing application state");

        Ok(Self {
            scryfall_storage,
            user_data_storage: user_data,
            icon,
            current_view: HOME,
            search_state: SearchState::default(),
            sync_view_state,
            view_about: Arc::new(AtomicBool::new(false)),
        })
    }
}

impl eframe::App for Oshibana {
    fn logic(&mut self, ctx: &Context, frame: &mut Frame) {
        static SAVE_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::S);

        let should_save = ctx.input_mut(|input| input.consume_shortcut(&SAVE_SHORTCUT));

        if should_save {
            self.user_data_storage.trigger_save();
        }

        if ctx.input(|i| i.viewport().close_requested()) && ctx.viewport_id() == ViewportId::ROOT {
            self.scryfall_storage
                .sync_handler
                .cancel_requested
                .store(true, Ordering::Relaxed);
            self.user_data_storage.save().expect("saved successfully");
        }

        if !self.scryfall_storage.is_ready()
            && !self.scryfall_storage.sync_handler.is_syncing()
            && !self.scryfall_storage.try_reload()
        {
            let progress_cb = self
                .sync_view_state
                .clone()
                .make_progress_cb(self.scryfall_storage.sync_handler.clone());

            self.scryfall_storage
                .clone()
                .trigger_sync(Arc::clone(&self.user_data_storage), progress_cb);
        }

        if ctx.input(|i| i.viewport().visible()).unwrap_or(true) {
            ctx.request_repaint();
        }

        (self.current_view.logic)(self, ctx, frame);
    }

    fn ui(&mut self, ui: &mut Ui, frame: &mut Frame) {
        if self.view_about.load(Ordering::Acquire) {
            let arc_clone = Arc::clone(&self.view_about);

            ui.show_viewport_deferred(
                ViewportId::from_hash_of("about"),
                ViewportBuilder::default()
                    .with_title("About Oshibana")
                    .with_active(true)
                    .with_inner_size([360.0, 250.0])
                    .with_icon(self.icon.clone()),
                move |ui, _| {
                    use crate::built::*;

                    if ui.input(|i| i.viewport().close_requested()) {
                        arc_clone.store(false, Ordering::Release);
                    }

                    fn labeled_link(ui: &mut Ui, label: &str, link: &str) {
                        ui.horizontal_wrapped(|ui| {
                            ui.label(label);
                            if ui.link(link).clicked()
                                && let Err(err) = opener::open_browser(link)
                            {
                                log::warn!("could not open {link}: {err}");
                            };
                        });
                    }

                    CentralPanel::default().show(ui, |ui| {
                        ui.heading("About Oshibana");
                        ui.label(format!("Authors: {}", PKG_AUTHORS.replace(":", ", ")));
                        labeled_link(ui, "Homepage: ", PKG_HOMEPAGE);
                        labeled_link(ui, "Repository: ", PKG_REPOSITORY);
                        ui.label(format!("License: {PKG_LICENSE}"));
                        ui.label(format!("Version: {PKG_VERSION}"));
                        ui.label(format!("Target: {TARGET}"));
                        ui.label(format!("Profile: {PROFILE}"));
                        ui.label(format!("Rust version: {RUSTC_VERSION}"));
                    });
                },
            );
        }

        if self.scryfall_storage.sync_handler.is_syncing() {
            self.sync_view_state
                .ui(&self.scryfall_storage.sync_handler, ui);
            return;
        }

        Panel::top("top_panel").show(ui, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Home").clicked() {
                        self.current_view = HOME;
                    }

                    if ui.button("Search").clicked() {
                        self.current_view = SEARCH;
                    }

                    if ui.button("Settings...").clicked() {
                        self.current_view = SETTINGS;
                    };

                    ui.separator();

                    if ui.button("Pull latest scryfall data").clicked() {
                        let progress_cb = self
                            .sync_view_state
                            .clone()
                            .make_progress_cb(self.scryfall_storage.sync_handler.clone());

                        self.scryfall_storage
                            .clone()
                            .trigger_sync(Arc::clone(&self.user_data_storage), progress_cb);
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
                        let fullscreen_state = ui.input(|input| input.viewport().maximized);

                        if let Some(state) = fullscreen_state {
                            ui.send_viewport_cmd(ViewportCommand::Maximized(!state));
                            ui.send_viewport_cmd(ViewportCommand::Decorations(state));
                        }
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About...").clicked() {
                        self.view_about.store(true, Ordering::Release);
                    }

                    if ui.button("Open logs folder").clicked() {
                        opener::reveal(storage::LOGS_DIR.as_path())
                            .inspect_err(|err| {
                                log::warn!("Could not open logs directory: {err}");
                            })
                            // discard error after logging
                            .ok();
                    }

                    if ui.button("Open data folder").clicked() {
                        opener::reveal(*storage::DATA_DIR)
                            .inspect_err(|err| {
                                log::warn!("Could not open data directory: {err}");
                            })
                            .ok();
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

        Panel::bottom("bottom_panel").show(ui, |ui| {
            ui.add_space(1.0);
            ui.horizontal(|ui| {
                ui.label(Local::now().format("%Y %B %e %r").to_string());

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    match (has_pending, is_saving) {
                        (_, true) => {
                            ui.spinner();
                            ui.label("Saving");
                        }

                        (true, false) => {
                            ui.label("Waiting for autosave");
                        }

                        (false, false) => {
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
        CentralPanel::default().show(ui, |ui| {
            (self.current_view.ui)(self, ui, frame);
        });
    }
}
