//! Different views that the application uses,

use std::any::Any;
use crate::app::Oshibana;
use eframe::Frame;
use egui::{Context, Ui};

pub mod home;
pub mod scryfall_sync;
pub mod search;
pub mod settings;

pub struct View {
    pub ui: fn(&mut Oshibana, &mut Ui, &mut Frame),
    pub logic: fn(&mut Oshibana, &Context, &mut Frame),
    pub menu: fn(&mut Oshibana, &mut Ui, &mut Frame),
    state: Box<dyn Any>
}

const fn ui_noop(_: &mut Oshibana, _: &mut Ui, _: &mut Frame) {}
const fn logic_noop(_: &mut Oshibana, _: &Context, _: &mut Frame) {}
