//! Different views that the application uses,

use egui::{Context, MenuBar, Panel, Ui, ViewportCommand};
use eframe::Frame;
use crate::app::Oshibana;

pub mod home;

pub static MENU_BAR_ID: &str = "top_bar";

#[derive(Copy, Clone)]
pub struct View {
    pub ui: fn(&mut Oshibana, &mut Ui, &mut Frame),
    pub logic: fn(&mut Oshibana, &Context, &mut Frame),
}

const fn ui_noop(_: &mut Oshibana, _: &mut Ui, _: &mut Frame) {}
const fn logic_noop(_: &mut Oshibana, _: &Context, _: &mut Frame) {}
