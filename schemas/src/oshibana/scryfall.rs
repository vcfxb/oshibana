//! Scryfall dataframe/parquet schema definition

use super::{enum_to_dt_enum, field, list};
use crate::scryfall::card::image_status::ImageStatus;
use crate::scryfall::card::languages::Language;
use crate::scryfall::card::layout::Layout;
use crate::scryfall::card::legalities::Legality;
use crate::scryfall::card::rarity::Rarity;
use crate::scryfall::card::related_card::Component;
use crate::scryfall::card::security_stamp::SecurityStamp;
use crate::scryfall::set::SetType;
use polars::prelude::{CategoricalPhysical, Categories, DataType, Schema, TimeUnit, TimeZone};
use std::sync::LazyLock;

/// UInt128s here are generally UUIDs, some of the fields are also bitflags.
pub static SCRYFALL_SCHEMA: LazyLock<Schema> = LazyLock::new(|| {
    let layout_enum = enum_to_dt_enum::<Layout>();
    let legality_enum = enum_to_dt_enum::<Legality>();

    let set_name_cat = Categories::new("set_names".into(), "sets".into(), CategoricalPhysical::U32);

    let set_name_dt = DataType::Categorical(set_name_cat.clone(), set_name_cat.mapping());

    let set_code_cat = Categories::new("set_codes".into(), "sets".into(), CategoricalPhysical::U32);

    let set_code_dt = DataType::Categorical(set_code_cat.clone(), set_code_cat.mapping());

    let image_uris_field = field(
        "image_uris",
        DataType::Struct(vec![
            field("small", DataType::String),
            field("normal", DataType::String),
            field("large", DataType::String),
            field("png", DataType::String),
            field("art_crop", DataType::String),
            field("border_crop", DataType::String),
        ]),
    );

    let datetime = DataType::Datetime(TimeUnit::Milliseconds, Some(TimeZone::UTC));

    Schema::from_iter(vec![
        field("id", DataType::UInt128),
        field("arena_id", DataType::UInt64),
        field("language", enum_to_dt_enum::<Language>()),
        field("mtgo_id", DataType::UInt64),
        field("mtgo_foil_id", DataType::UInt64),
        field("multiverse_ids", list(DataType::UInt64)),
        field("resource_id", DataType::String),
        field("tcgplayer_id", DataType::UInt64),
        field("tcgplayer_etched_id", DataType::UInt64),
        field("cardmarket_id", DataType::UInt64),
        field("layout", layout_enum.clone()),
        field("oracle_id", DataType::UInt128),
        field("prints_search_uri", DataType::String),
        field("rulings_uri", DataType::String),
        field("scryfall_uri", DataType::String),
        field("uri", DataType::String),
        field(
            "related_cards",
            DataType::List(Box::new(DataType::Struct(vec![
                field("id", DataType::UInt128),
                field("component", enum_to_dt_enum::<Component>()),
            ]))),
        ),
        field(
            "card_faces",
            DataType::List(Box::new(DataType::Struct(vec![
                field("artist", DataType::String),
                field("artist_id", DataType::UInt128),
                field("cmc", DataType::Float32),
                // bitflags bytes
                field("color_indicator", DataType::UInt8),
                field("colors", DataType::UInt8),
                field("defense", DataType::String),
                field("flavor_text", DataType::String),
                field("illustration_id", DataType::UInt128),
                image_uris_field.clone(),
                field("layout", layout_enum.clone()),
                field("loyalty", DataType::String),
                field("mana_cost", DataType::String),
                field("name", DataType::String),
                field("oracle_id", DataType::UInt128),
                field("oracle_text", DataType::String),
                field("power", DataType::String),
                field("printed_name", DataType::String),
                field("printed_type_line", DataType::String),
                field("toughness", DataType::String),
                field("type_line", DataType::String),
                field("watermark", DataType::String),
            ]))),
        ),
        field("cmc", DataType::Float32),
        field("color_identity", DataType::UInt8),
        field("color_indicator", DataType::UInt8),
        field("colors", DataType::UInt8),
        field("defense", DataType::String),
        field("edhrec_rank", DataType::UInt64),
        field("game_changer", DataType::Boolean),
        field("hand_modifier", DataType::String),
        field("keywords", list(DataType::String)),
        field(
            "legalities",
            DataType::Struct(vec![
                field("standard", legality_enum.clone()),
                field("historic", legality_enum.clone()),
                field("timeless", legality_enum.clone()),
                field("pioneer", legality_enum.clone()),
                field("modern", legality_enum.clone()),
                field("legacy", legality_enum.clone()),
                field("pauper", legality_enum.clone()),
                field("vintage", legality_enum.clone()),
                field("commander", legality_enum.clone()),
                field("oathbreaker", legality_enum.clone()),
                field("standard_brawl", legality_enum.clone()),
                field("brawl", legality_enum.clone()),
                field("alchemy", legality_enum.clone()),
                field("pauper_commander", legality_enum.clone()),
                field("premodern", legality_enum.clone()),
            ]),
        ),
        field("life_modifier", DataType::String),
        field("loyalty", DataType::String),
        field("mana_cost", DataType::String),
        field("name", DataType::String),
        field("oracle_text", DataType::String),
        field("power", DataType::String),
        field("produced_mana", DataType::UInt8),
        field("reserved", DataType::Boolean),
        field("toughness", DataType::String),
        field("type_line", DataType::String),
        field("artist", DataType::String),
        field("artist_ids", list(DataType::UInt128)),
        field("attraction_lights", list(DataType::UInt8)),
        field("booster", DataType::Boolean),
        field("border_color", DataType::String),
        field("card_back_id", DataType::UInt128),
        field("collector_number", DataType::String),
        field("content_warning", DataType::Boolean),
        field("digital", DataType::Boolean),
        field("finishes", DataType::UInt8), // Bitflag again
        field("flavor_name", DataType::String),
        field("flavor_text", DataType::String),
        field("frame_effects", DataType::UInt32), // bitflag here too
        field("frame", DataType::String),
        field("full_art", DataType::Boolean),
        field("games", DataType::UInt32), // bitflag here as well
        field("highres_image", DataType::Boolean),
        field("illustration_id", DataType::UInt128),
        field("image_status", enum_to_dt_enum::<ImageStatus>()),
        image_uris_field.clone(),
        field("oversized", DataType::Boolean),
        field(
            "prices",
            DataType::Struct(vec![
                field("usd", DataType::Float32),
                field("usd_foil", DataType::Float32),
                field("usd_etched", DataType::Float32),
                field("eur", DataType::Float32),
                field("eur_foil", DataType::Float32),
                field("tix", DataType::Float32),
            ]),
        ),
        field("printed_name", DataType::String),
        field("printed_text", DataType::String),
        field("printed_type_line", DataType::String),
        field("promo", DataType::Boolean),
        field("promo_types", list(DataType::String)),
        field(
            "purchase_uris",
            DataType::Struct(vec![
                field("tcgplayer", DataType::String),
                field("cardmarket", DataType::String),
                field("cardhoarder", DataType::String),
            ]),
        ),
        field("rarity", enum_to_dt_enum::<Rarity>()),
        field(
            "related_uris",
            DataType::Struct(vec![
                field("gatherer", DataType::String),
                field("edhrec", DataType::String),
            ]),
        ),
        field("released_at", datetime.clone()),
        field("reprint", DataType::Boolean),
        field("scryfall_set_uri", DataType::String),
        field("set_name", set_name_dt.clone()),
        field("set_search_uri", DataType::String),
        field("set_type", enum_to_dt_enum::<SetType>()),
        field("set_uri", DataType::String),
        field("set", set_code_dt.clone()),
        field("set_id", DataType::UInt128),
        field("story_spotlight", DataType::Boolean),
        field("textless", DataType::Boolean),
        field("variation", DataType::Boolean),
        field("variation_of", DataType::UInt128),
        field("security_stamp", enum_to_dt_enum::<SecurityStamp>()),
        field("watermark", DataType::String),
    ])
});
