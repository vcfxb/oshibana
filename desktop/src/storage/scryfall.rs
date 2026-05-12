//! Scryfall-data is stored

use crate::storage::DIRECTORIES;
use polars::prelude::LazyFrame;
use std::path::PathBuf;
use std::sync::LazyLock;

static SCRYFALL_DATA_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| DIRECTORIES.data_dir().join("scryfall"));

static SCRYFALL_DATA_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| SCRYFALL_DATA_PATH.join("data.parquet"));

pub struct ScryfallStorage {
    lazy_frame: LazyFrame,
}

impl ScryfallStorage {}
