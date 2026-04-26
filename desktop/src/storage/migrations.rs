//! Migrations on the sqlite db that oshibana stores locally.

use rusqlite_migration::{Migrations, M};

const MIGRATION_SLICE: &[M] = &[
    M::up(r#"CREATE TABLE scryfall_data (
        id BLOB PRIMARY KEY,
        set_code TEXT NOT NULL,
        collector_number TEXT NOT NULL,
        json BLOB NOT NULL
    );"#).down("DROP TABLE scryfall_data;"),
];

const MIGRATIONS: Migrations = Migrations::from_slice(MIGRATION_SLICE);
