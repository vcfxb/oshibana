//! Storage & filesystem handling for oshibana

pub mod scryfall;
pub mod user_data;

use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use directories::ProjectDirs;

pub static DIRECTORIES: LazyLock<ProjectDirs> =
    LazyLock::new(|| ProjectDirs::from("org.vcfxb", "Venus Xeon-Blonde", "Oshibana").unwrap());

pub static LOGS_DIR: LazyLock<PathBuf> = LazyLock::new(|| DIRECTORIES.cache_dir().join("logs"));

pub static DATA_DIR: LazyLock<&Path> = LazyLock::new(|| DIRECTORIES.data_dir());

