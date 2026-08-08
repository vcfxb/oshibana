//! Card view

use eframe::Frame;
use egui::Ui;
use uuid::Uuid;
use crate::app::Oshibana;
use crate::view::{logic_noop, ui_noop, View};

pub fn card(view: View, scryfall_id: Uuid) -> View {
    View {
        ui: card_view_ui,
        logic: logic_noop,
        menu: ui_noop,
        state: Box::new(CardViewState { scryfall_id, last_view: view}),
    }
}

pub struct CardViewState {
    scryfall_id: Uuid,
    last_view: View,
}

fn card_view_ui(app: &mut Oshibana, ui: &mut Ui, _: &mut Frame) {
    let state: &mut CardViewState = app.current_view.state.downcast_mut().unwrap();
    
    

}
