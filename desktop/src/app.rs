pub mod autosave;
pub mod save;
pub mod scryfall_pull;

use crate::app::scryfall_pull::ScryfallPullStatus;
use crate::views::View;
use clients::scryfall::ScryfallClient;
use eframe::Frame;
use egui::{
    Context, IconData, Key, KeyboardShortcut, Modifiers, Panel, ProgressBar, Theme,
    ViewportBuilder, ViewportCommand, ViewportId, containers::menu::MenuBar,
};
use std::sync::Arc;
use std::sync::atomic::Ordering;

pub struct Oshibana {
    // autosave: AutoSaveState,
    icon: Arc<IconData>,
    scryfall_client: ScryfallClient,
    current_view: View,
    scryfall_pull_status: Arc<ScryfallPullStatus>,
}

impl Oshibana {
    pub fn new(_: &eframe::CreationContext<'_>, icon: Arc<IconData>) -> anyhow::Result<Self> {
        Ok(Self {
            // user_data: storage::load_user_data()?,
            icon,
            scryfall_client: ScryfallClient::new(),
            current_view: View::Home,
            scryfall_pull_status: Arc::new(ScryfallPullStatus::default()),
        })
    }

    fn tigger_scryfall_pull(&self) {
        log::info!("Scryfall pull triggered");

        // Don't start a new task if we're already in progress
        let atomic = &self.scryfall_pull_status.in_progress;

        let scryfall_client_clone = self.scryfall_client.clone();

        todo!();
        tokio::spawn(async move {});
    }

    // fn run_search(&self, ctx: egui::Context) {
    //     let query = self.search_query.clone();
    //     let client = self.scryfall_client.clone();
    //     let state = self.search_state.clone();
    //
    //     {
    //         let mut s = state.lock().unwrap();
    //         *s = SearchState::Searching;
    //     }
    //
    //     tokio::spawn(async move {
    //         todo!();
    //         // match client.search_cards(&query).await {
    //         //     Ok(list) => {
    //         //         let mut s = state.lock().unwrap();
    //         //         *s = SearchState::Results(list.data);
    //         //     }
    //         //     Err(e) => {
    //         //         let mut s = state.lock().unwrap();
    //         //         *s = SearchState::Error(e.to_string());
    //         //     }
    //         // }
    //         ctx.request_repaint();
    //     });
    // }
}

impl eframe::App for Oshibana {
    fn logic(&mut self, ctx: &Context, _frame: &mut Frame) {
        static QUIT_SHORTCUT: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::Q);

        // avoid deadlocking here by calling send_viewport_command outside
        let should_quit = ctx.input_mut(|input_state| input_state.consume_shortcut(&QUIT_SHORTCUT));

        if should_quit {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut Frame) {
        Panel::top("menu_bar_panel").show_inside(ui, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Pull latest scryfall data").clicked() {
                        self.tigger_scryfall_pull();
                    }

                    ui.separator();

                    if ui.button("Save").clicked() {
                        todo!()
                    }

                    // let mut current_autosave = self.autosave.load(Ordering::Acquire);
                    // let toggle_autosave = ui.toggle_value(&mut current_autosave, "Autosave");

                    // if toggle_autosave.clicked() {
                    //     self.autosave.store(current_autosave, Ordering::Release);
                    //     todo!()
                    // }

                    ui.separator();

                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(ViewportCommand::Close);
                    }
                });

                ui.menu_button("Import", |ui| {
                    ui.menu_button("From Moxfield", |ui| {
                        ui.button("Deck...");
                        ui.button("Collection...");
                    })
                })
            })
        });

        // // Left panel nav
        // egui::Panel::left("navigation").show_inside(ui, |ui| {
        //     ui.vertical(|ui| {
        //         ui.selectable_value(&mut self.current_view, View::Home, "🏠 Home");
        //         ui.selectable_value(&mut self.current_view, View::Search, "🔍 Search");
        //         ui.selectable_value(&mut self.current_view, View::Collection, "🎴 Collection");
        //         ui.selectable_value(&mut self.current_view, View::Decks, "🃏 Decks");
        //         ui.separator();
        //         ui.selectable_value(&mut self.current_view, View::Settings, "⚙ Settings");
        //     });
        // });
        //
        // egui::CentralPanel::default().show_inside(ui, |ui| {
        //     // match self.current_view {
        //     //     Page::Home => {
        //     //         ui.heading("Welcome to Oshibana");
        //     //         ui.label("Manage your MTG collection with ease.");
        //     //
        //     //     }
        //     //     Page::Search => {
        //     //         ui.heading("Search Cards");
        //     //         ui.horizontal(|ui| {
        //     //             let response = ui.text_edit_singleline(&mut self.search_query);
        //     //             if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) || ui.button("Search").clicked() {
        //     //                 self.run_search(ui.ctx().clone());
        //     //             }
        //     //         });
        //     //
        //     //         ui.separator();
        //     //
        //     //         let state = self.search_state.lock().unwrap();
        //     //         match &*state {
        //     //             SearchState::Idle => {
        //     //                 ui.label("Enter a query to start searching.");
        //     //             }
        //     //             SearchState::Searching => {
        //     //                 ui.spinner();
        //     //                 ui.label("Searching Scryfall...");
        //     //             }
        //     //             SearchState::Results(cards) => {
        //     //                 if cards.is_empty() {
        //     //                     ui.label("No cards found.");
        //     //                 } else {
        //     //                     egui::ScrollArea::vertical().show(ui, |ui| {
        //     //                         for card in cards {
        //     //                             ui.group(|ui| {
        //     //                                 ui.horizontal(|ui| {
        //     //                                     ui.label(&card.name);
        //     //                                     ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        //     //                                         if let Some(cost) = &card.mana_cost {
        //     //                                             ui.label(cost);
        //     //                                         }
        //     //                                     });
        //     //                                 });
        //     //                                 ui.label(&card.type_line);
        //     //                             });
        //     //                         }
        //     //                     });
        //     //                 }
        //     //             }
        //     //             SearchState::Error(e) => {
        //     //                 ui.colored_label(egui::Color32::RED, format!("Error: {}", e));
        //     //             }
        //     //         }
        //     //     }
        //     //     Page::Collection => {
        //     //         ui.heading("Your Collection");
        //     //         ui.label("Collection view coming soon...");
        //     //     }
        //     //     Page::Decks => {
        //     //         ui.heading("Your Decks");
        //     //         ui.label("Deck builder coming soon...");
        //     //     }
        //     //     Page::Settings => {
        //     //         ui.heading("Settings");
        //     //         ui.label("Settings coming soon...");
        //     //     }
        //     // }
        // });
    }
}
