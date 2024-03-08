use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use std::default::Default;
use crate::data::datapack::DatapackFormat;
use crate::data::elements::carver::CarverData;
use crate::data::elements::element::{DataElement, VersionedData};
use crate::data::util::{BlockState, ItemNBT, ItemStack, ResourceLocation};

#[derive(Debug, Copy, Clone)]
pub struct BiomeElement {
    name: ResourceLocation,

    // Data which is always the same for all formats
    temperature: f32,
    temperature_modifier: TemperatureModifier,
    downfall: f32,
    effects: Effect,
    carvers: CarverList,
    features: FeatureList,

    // Format-specific data
    precipitation: Precipitation
}

impl BiomeElement {

}

impl DataElement for BiomeElement {
    fn serialize(&self, format: DatapackFormat) -> &'static str {
        use DatapackFormat::*;
        match format {
            FORMAT6 | FORMAT7 => {}
            FORMAT8 | FORMAT9 => {}
            FORMAT10 => {}
            FORMAT12 | FORMAT15 | FORMAT18 | FORMAT26 | FORMAT34 => {}
        }
        todo!()
    }

    fn deserialize(&self, format: DatapackFormat, json: &str) {
        todo!()
    }
}

///////////////////////////////////////////
//------ Biome Data Internal Types ------//
///////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
struct Precipitation {
    has_precipitation: Option<bool>,
    precipitation: Option<LegacyPrecipitationCategory>
}

impl VersionedData<PrecipitationVariant> for Precipitation {
    fn get_value(&self, format: DatapackFormat) -> Option<PrecipitationVariant> {
        if format >= DatapackFormat::FORMAT12 {
            if let Some(has_precipitation) = self.has_precipitation {
                Some(PrecipitationVariant::Boolean(has_precipitation))
            }
            else { None }
        }
        else if let Some(precipitation) = self.precipitation {
            Some(PrecipitationVariant::Legacy(precipitation))
        }
        else { None }
    }
}

#[derive(Debug, Copy, Clone)]
enum PrecipitationVariant {
    Boolean(bool),
    Legacy(LegacyPrecipitationCategory)
}

//////////////////////////////////////////
//------ Biome Data Serialization ------//
//////////////////////////////////////////

#[derive(Debug)]
pub enum BiomeData {
    Format12(BiomeDataFormat12),
    Format10(BiomeDataFormat10),
    Format8(BiomeDataFormat8),
    Format6(BiomeDataFormat6)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomeDataFormat12 {
    has_precipitation: bool,
    temperature: f32,
    #[serde(default)]
    temperature_modifier: TemperatureModifier,
    downfall: f32,
    effects: Effect,
    carvers: CarverList,
    features: FeatureList,
    #[serde(default)]
    creature_spawn_probability: Option<f32>
    // TODO: spawners entry and data
    // TODO: spawn costs
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomeDataFormat10 {
    precipitation: LegacyPrecipitationCategory,
    temperature: f32,
    #[serde(default)]
    temperature_modifier: TemperatureModifier,
    downfall: f32,
    effects: Effect,
    carvers: CarverList,
    features: FeatureList,
    #[serde(default)]
    creature_spawn_probability: Option<f32>
    // TODO: spawners entry and data
    // TODO: spawn costs
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomeDataFormat8 {
    precipitation: LegacyPrecipitationCategory,
    temperature: f32,
    #[serde(default)]
    temperature_modifier: TemperatureModifier,
    downfall: f32,
    effects: Effect,
    carvers: CarverList,
    features: FeatureList,
    #[serde(default)]
    creature_spawn_probability: Option<f32>
    // TODO: spawners entry and data
    // TODO: spawn costs
    // TODO: biome category
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BiomeDataFormat6 {
    player_spawn_friendly: bool,
    depth: i32,
    scale: i32,
    precipitation: LegacyPrecipitationCategory,
    temperature: f32,
    #[serde(default)]
    temperature_modifier: TemperatureModifier,
    downfall: f32,
    effects: Effect,
    carvers: CarverList,
    placed_features: FeatureList,
    #[serde(default)]
    creature_spawn_probability: Option<f32>
    // TODO: spawners entry and data
    // TODO: spawn costs
    // TODO: biome category
    // TODO: surface builder
    // TODO: structure starts
}

///////////////////////////////////////////////////////
//------ Biome Data Serialization Helper Types ------//
///////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone, Serialize, Deserialize, EnumString)]
pub enum TemperatureModifier {
    #[strum(to_string = "translate.biome.temperature_modifier.none")]
    #[serde(alias = "none")]
    None,
    #[strum(to_string = "translate.biome.temperature_modifier.frozen")]
    #[serde(alias = "frozen")]
    Frozen
}
impl Default for TemperatureModifier { fn default() -> Self { TemperatureModifier::None } }

#[derive(Debug, Copy, Clone, Serialize, Deserialize, EnumString)]
pub enum GrassColorModifier {
    #[strum(to_string = "translate.biome.grass_color_modifier.none")]
    #[serde(alias = "none")]
    None,
    #[strum(to_string = "translate.biome.grass_color_modifier.dark_forest")]
    #[serde(alias = "dark_forest")]
    DarkForest,
    #[strum(to_string = "translate.biome.grass_color_modifier.swamp")]
    #[serde(alias = "swamp")]
    Swamp
}
impl Default for GrassColorModifier { fn default() -> Self { GrassColorModifier::None } }

// Used until Format 10
#[derive(Debug, Copy, Clone, Serialize, Deserialize, EnumString)]
pub enum LegacyPrecipitationCategory {
    #[strum(to_string = "translate.biome.temperature_modifier.none")]
    #[serde(alias = "none")]
    None,
    #[strum(to_string = "translate.biome.temperature_modifier.frozen")]
    #[serde(alias = "rain")]
    Rain,
    #[strum(to_string = "translate.biome.temperature_modifier.frozen")]
    #[serde(alias = "snow")]
    Snow
}
impl Default for LegacyPrecipitationCategory { fn default() -> Self { LegacyPrecipitationCategory::Rain } }

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct Effect {
    fog_color: i32,
    sky_color: i32,
    water_color: i32,
    water_fog_color: i32,
    #[serde(default)]
    foliage_color: Option<i32>,
    #[serde(default)]
    grass_color: Option<i32>,
    #[serde(default)]
    grass_color_modifier: GrassColorModifier,
    #[serde(default)]
    particle: Option<Particle>
    // TODO: Rest of spec
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
enum Particle {
    #[serde(rename = "block")]
    Block(BlockState),
    #[serde(rename = "block_marker")]
    BlockMarker(BlockState),
    #[serde(rename = "falling_dust")]
    FallingDust(BlockState),
    #[serde(rename = "item")]
    Item(ItemStack), // TODO: This is versioned - figure out how to represent that
    #[serde(rename = "dust")]
    Dust {
        color: (f32, f32, f32),
        scale: f32
    },
    #[serde(rename = "dust_color_transition")]
    DustColorTransition {
        #[serde(rename = "fromColor")]
        from_color: (f32, f32, f32),
        #[serde(rename = "toColor")]
        to_color: (f32, f32, f32)
    },
    #[serde(rename = "sculk_charge")]
    SculkCharge {
        roll: f32
    },
    #[serde(rename = "vibration")]
    Vibration {
        destination: VibrationPositionSource,
        arrival_in_ticks: i32
    },
    #[serde(rename = "shriek")]
    Shriek {
        delay: i32
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum VibrationPositionSource {
    #[serde(rename = "block")]
    Block {
        pos: (i32, i32, i32)
    },
    #[serde(rename = "entity")]
    Entity {
        source_entity: (i32, i32, i32, i32),
        y_offset: f32
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct CarverList {
    air: Carver,
    liquid: Carver
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum Carver {
    ID(ResourceLocation),
    CARVER(CarverData),
    LIST(Vec<CarverData>)
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct FeatureList {
    // TODO: Implementation of the 11 steps - needs custom serialization
    // Feature placement has 11 steps (more info on wiki) in order
    // Feature placement must also be in the same order across all biomes
    //      per step so that'll be difficult to ensure
}