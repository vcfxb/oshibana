
// #![windows_subsystem = "windows"]

pub mod storage;
pub mod views;
pub mod app;

use std::fs;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};
use directories::ProjectDirs;
use eframe::NativeOptions;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::onstartup::OnStartUpTrigger;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::filter::threshold::ThresholdFilter;
use log::LevelFilter;
use rusqlite::Connection;
use crate::app::Oshibana;

static DIRECTORIES: LazyLock<ProjectDirs> = LazyLock::new(|| {
    ProjectDirs::from("org.vcfxb", "Venus Xeon-Blonde", "Oshibana").unwrap()
});

static DB_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    DIRECTORIES.data_dir().join("oshibana-db.sqlite")
});


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stdout_appender = ConsoleAppender::builder()
        .build();

    let log_file_dir = DIRECTORIES.cache_dir();

    // create log file cache path if it doesn't exist
    eprintln!("Creating {} if it doesn't exist to store logs.", log_file_dir.display());
    fs::create_dir_all(log_file_dir)?;

    let archive_pattern = log_file_dir.join("archived-oshibana-{}.log");

    let rolling_log_policy = Box::new(
        CompoundPolicy::new(
            Box::new(OnStartUpTrigger::new(0)),
            Box::new(FixedWindowRoller::builder().build(archive_pattern.to_str().unwrap(), 3)?),
        )
    );

    let rolling_file_appender = RollingFileAppender::builder()
        .build(log_file_dir.join("oshibana.log"), rolling_log_policy)?;

    let log4rs_config = log4rs::Config::builder()
        .appender(Appender::builder().filter(Box::new(ThresholdFilter::new(LevelFilter::Info))).build("stdout", Box::new(stdout_appender)))
        .appender(Appender::builder().build("file", Box::new(rolling_file_appender)))
        // silence lots of spurious logs for other crates
        .logger(Logger::builder().build("wgpu_hal", LevelFilter::Warn))
        .logger(Logger::builder().build("wgpu_core", LevelFilter::Info))
        .logger(Logger::builder().build("naga", LevelFilter::Info))
        .logger(Logger::builder().build("eframe", LevelFilter::Info))
        .logger(Logger::builder().build("egui_wgpu", LevelFilter::Warn))
        .build(Root::builder().appenders(["stdout", "file"]).build(LevelFilter::Debug))?;

    log4rs::init_config(log4rs_config)?;
    log::info!("started logger (stderr, dir: {})", log_file_dir.display());

    let data_dir = DIRECTORIES.data_dir();
    log::debug!("creating data dir if doesn't exist");
    fs::create_dir_all(data_dir)?;
    let mut db_connection = Connection::open(DB_PATH.as_path())?;
    log::info!("migrating db");
    storage::migrations::MIGRATIONS.to_latest(&mut db_connection)?;

    let native_options = NativeOptions {
        vsync: true,
        centered: true,
        dithering: true,
        .. NativeOptions::default()
    };

    eframe::run_native("Oshibana", native_options, Box::new(|cc| {
        Ok(Box::new(Oshibana::new(cc, db_connection)?))
    }))?;

    Ok(())
}
