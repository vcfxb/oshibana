//! Storage management for oshibana -- handles fs communication etc.

use std::path::PathBuf;
use std::sync::LazyLock;
use directories::ProjectDirs;

pub static DIRECTORIES: LazyLock<ProjectDirs> = LazyLock::new(|| {
    ProjectDirs::from("org.vcfxb", "Venus Xeon-Blonde", "Oshibana").unwrap()
});

pub static LOGS_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    DIRECTORIES.cache_dir().join("logs")
});

pub static SCRYFALL_DATA_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    DIRECTORIES.data_dir().join("scryfall-cache.messagepack")
});

pub static USER_DATA_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    DIRECTORIES.data_dir().join("user-data.messagepack")
});
