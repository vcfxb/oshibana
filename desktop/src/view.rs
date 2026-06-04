//! Different views that the application uses,

use crate::app::Oshibana;
use eframe::Frame;
use egui::{Context, Ui};

pub mod home;
pub mod settings;

#[derive(Copy, Clone)]
pub struct View {
    pub ui: fn(&mut Oshibana, &mut Ui, &mut Frame),
    pub logic: fn(&mut Oshibana, &Context, &mut Frame),
    pub menu: fn(&mut Oshibana, &mut Ui, &mut Frame),
}

const fn ui_noop(_: &mut Oshibana, _: &mut Ui, _: &mut Frame) {}
const fn logic_noop(_: &mut Oshibana, _: &Context, _: &mut Frame) {}
