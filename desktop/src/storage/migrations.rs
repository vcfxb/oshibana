//! Migrations on the sqlite db that oshibana stores locally.

use rusqlite_migration::{Migrations, M};

const SCRYFALL_DB_MIGRATION_SLICE: &[M] = &[
    M::up(r#"CREATE TABLE scryfall_data (
        id BLOB PRIMARY KEY,
        set_code TEXT NOT NULL,
        collector_number TEXT NOT NULL,
        json BLOB NOT NULL,
        oracle_id BLOB,
        name TEXT NOT NULL,
        cmc REAL NOT NULL,
        color_identitity_flags NUMBER NOT NULL,
        color_flags NUMBER,
        type_line TEXT NOT NULL,
        lang TEXT NOT NULL
    );"#).down("DROP TABLE scryfall_data;"),

    // KV store for keeping info about when scryfall data was pulled, etc.
    M::up(r#"CREATE TABLE kv_store (
        key TEXT PRIMARY KEY,
        value TEXT NOT NULL
    );"#).down("DROP TABLE kv_store;"),
];

const OSHIBANA_DB_MIGRATION_SLICE: &[M] = &[
    // KV store for keeping info about config/sync etc.
    M::up(r#"CREATE TABLE kv_store (
        key TEXT PRIMARY KEY,
        value TEXT NOT NULL
    );"#).down("DROP TABLE kv_store;"),

    M::up(r#"CREATE TABLE collection (
        quantity INTEGER NOT NULL,
        scryfall_id BLOB NOT NULL
        
    );"#).down("DROP TABLE collection;")
];

// pub const MIGRATIONS: Migrations = Migrations::from_slice(MIGRATION_SLICE);
