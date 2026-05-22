use std::sync::LazyLock;
use enumflags2::bitflags;
use polars::chunked_array::builder::CategoricalChunkedBuilder;
use polars::datatypes::{Categorical8Type, DataType};
use polars::error::PolarsResult;
use polars::prelude::{CategoricalChunked, PlSmallStr};
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoStaticStr};
use crate::enum_to_dt_enum;
use crate::traits::builder::PolarsBuilder;
use crate::traits::map_type::MapPolarsType;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Copy, Clone, EnumIter, IntoStaticStr)]
#[serde(rename_all = "lowercase")]
#[bitflags]
#[repr(u32)]
pub enum FrameEffect {
    Legendary,
    Miracle,
    Enchantment,
    Draft,
    Devoid,
    Tombstone,
    Colorshifted,
    Inverted,
    SunMoonDfc,
    CompassLandDfc,
    OriginPwDfc,
    MoonEldraziDfc,
    WaxingAndWaningMoonDfc,
    Showcase,
    ExtendedArt,
    Companion,
    Etched,
    Snow,
    Lesson,
    ShatteredGlass,
    ConvertDfc,
    FanDfc,
    UpsideDownDfc,
    Spree,
}

pub static FRAME_EFFECT_DT: LazyLock<DataType> = LazyLock::new(|| enum_to_dt_enum::<FrameEffect>());

impl MapPolarsType for FrameEffect {
    type StaticPolarsType = Categorical8Type;
    type Builder = CategoricalChunkedBuilder<Categorical8Type>;

    fn dt() -> DataType {
        FRAME_EFFECT_DT.clone()
    }
}

impl PolarsBuilder<FrameEffect> for CategoricalChunkedBuilder<Categorical8Type> {
    type ChunkedType = CategoricalChunked<Categorical8Type>;

    fn new() -> Self {
        CategoricalChunkedBuilder::new(PlSmallStr::EMPTY, FRAME_EFFECT_DT.clone())
    }

    fn append(&mut self, val: FrameEffect) -> PolarsResult<()> {
        self.append_str(val.into())
    }

    fn append_null(&mut self) {
        self.append_null()
    }

    fn finish(self) -> PolarsResult<Self::ChunkedType> {
        Ok(self.finish())
    }
}
