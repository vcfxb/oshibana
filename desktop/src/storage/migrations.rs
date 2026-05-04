//! Migrations on the sqlite db that oshibana stores locally.

use rusqlite_migration::{Migrations, M};

const MIGRATION_SLICE: &[M] = &[
    M::up(r#"CREATE TABLE scryfall_data (
        id BLOB PRIMARY KEY,
        set_code TEXT NOT NULL,
        collector_number TEXT NOT NULL,
        json BLOB NOT NULL
    );"#).down("DROP TABLE scryfall_data;"),

    M::up(r#"
        ALTER TABLE scryfall_data ADD oracle_id BLOB;
        ALTER TABLE scryfall_data ADD name TEXT NOT NULL;
        ALTER TABLE scryfall_data ADD cmc REAL NOT NULL;
        ALTER TABLE scryfall_data ADD color_identitity_flags NUMBER NOT NULL;
        ALTER TABLE scryfall_data ADD color_flags NUMBER;
        ALTER TABLE scryfall_data ADD type_line TEXT NOT NULL;
        ALTER TABLE scryfall_data ADD lang TEXT NOT NULL;
    "#).down(r#"
        ALTER TABLE scryfall_data DROP COLUMN oracle_id;
        ALTER TABLE scryfall_data DROP COLUMN name;
        ALTER TABLE scryfall_data DROP COLUMN cmc;
        ALTER TABLE scryfall_data DROP COLUMN color_identity_flags;
        ALTER TABLE scryfall_data DROP COLUMN color_flags;
        ALTER TABLE scryfall_data DROP COLUMN type_line;
        ALTER TABLE scryfall_data DROP COLUMN lang;
    "#),

    M::up(r#"CREATE TABLE kv_store (
        key TEXT PRIMARY KEY,
        value TEXT NOT NULL
    );"#).down("DROP TABLE kv_store;"),


];

pub const MIGRATIONS: Migrations = Migrations::from_slice(MIGRATION_SLICE);
