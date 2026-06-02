
// release builds don't log / show terminal.
#![cfg_attr(feature = "prod", windows_subsystem = "windows")]

pub mod app;
pub mod storage;
pub mod view;

use crate::app::Oshibana;
use crate::storage::LOGS_DIR;
use eframe::NativeOptions;
use egui::{IconData, Theme, ViewportBuilder};
use image::GenericImageView;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::onstartup::OnStartUpTrigger;
use log4rs::config::{Appender, Root};
use log4rs::filter::threshold::ThresholdFilter;
use std::{fs, panic};
use std::sync::Arc;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let stdout_appender = ConsoleAppender::builder().build();

    let log_file_dir = LOGS_DIR.as_path();

    // create log file cache path if it doesn't exist
    eprintln!(
        "Creating {} if it doesn't exist to store logs.",
        log_file_dir.display()
    );
    fs::create_dir_all(log_file_dir)?;

    let archive_pattern = log_file_dir.join("archived-oshibana-{}.log");

    let rolling_log_policy = Box::new(CompoundPolicy::new(
        Box::new(OnStartUpTrigger::new(0)),
        Box::new(FixedWindowRoller::builder().build(archive_pattern.to_str().unwrap(), 3)?),
    ));

    let rolling_file_appender = RollingFileAppender::builder()
        .build(log_file_dir.join("oshibana.log"), rolling_log_policy)?;

    let log4rs_config = log4rs::Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Info)))
                .build("stdout", Box::new(stdout_appender)),
        )
        .appender(Appender::builder().build("file", Box::new(rolling_file_appender)))
        // silence lots of spurious logs for other crates
        // .logger(Logger::builder().build("wgpu_hal", LevelFilter::Warn))
        // .logger(Logger::builder().build("wgpu_core", LevelFilter::Info))
        // .logger(Logger::builder().build("naga", LevelFilter::Info))
        // .logger(Logger::builder().build("eframe", LevelFilter::Info))
        // .logger(Logger::builder().build("egui_wgpu", LevelFilter::Warn))
        .build(
            Root::builder()
                .appenders(["stdout", "file"])
                .build(LevelFilter::Debug),
        )?;

    log4rs::init_config(log4rs_config)?;
    log::info!("started logger (stderr, dir: {})", log_file_dir.display());

    // override the panic hook to log as well.
    let panic_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        log::error!("panic: {info}");
        panic_hook(info);
    }));

    let data_dir = *storage::DATA_DIR;
    log::info!("creating data dir if doesn't exist: {}", data_dir.display());
    fs::create_dir_all(data_dir)?;

    let icon_data = {
        let png_bytes: &[u8] = match egui::Context::default().system_theme() {
            None | Some(Theme::Dark) => include_bytes!("../assets/favicon-light.png"),
            Some(Theme::Light) => include_bytes!("../assets/favicon-dark.png"),
        };

        let png = image::load_from_memory(png_bytes).expect("failed to load icon");
        let (width, height) = png.dimensions();

        IconData {
            rgba: png.into_rgba8().into_raw(),
            width,
            height,
        }
    };

    let icon_data_arc = Arc::new(icon_data);

    let native_options = NativeOptions {
        vsync: true,
        centered: true,
        dithering: true,
        // persist window size and location
        persist_window: true,
        viewport: ViewportBuilder::default()
            .with_icon(icon_data_arc.clone())
            .with_active(true),
        ..NativeOptions::default()
    };

    eframe::run_native(
        "Oshibana",
        native_options,
        Box::new(|cc| {
            egui_material_icons::initialize(&cc.egui_ctx);
            Ok(Box::new(Oshibana::new(cc, icon_data_arc)?))
        }),
    ).unwrap();

    Ok(())
}
