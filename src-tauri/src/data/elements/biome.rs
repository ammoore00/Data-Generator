use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use std::default::Default;
use tauri::regex::Regex;
use crate::data::elements::carver::CarverData;
use crate::data::elements::element::NamedDataElement;
use crate::data::util::{BlockState, ItemStack, ResourceLocation};

lazy_static! {
    static ref BIOME_REG: Regex = Regex::new(r"data/[a-z0-9_.-]+/worldgen/biome/([a-z0-9/_.-]+)\.json").unwrap();
}

#[derive(Debug, Clone)]
pub struct BiomeElement {
    name: ResourceLocation,
    shared_data: BiomeSharedData,
    format_data: BiomeFormatData
}

impl BiomeElement {
    pub fn new(name: ResourceLocation, shared_data: BiomeSharedData, format_data: BiomeFormatData) -> Self {
        BiomeElement{name, shared_data, format_data}
    }
}

impl NamedDataElement for BiomeElement {
    fn serialize(&self) -> String {
        todo!()
    }

    fn deserialize(name: ResourceLocation, json: String) -> serde_json::Result<Box<Self>> {
        let shared_data: BiomeSharedData = serde_json::from_str(json.as_str())?;
        let format_data: BiomeFormatData = serde_json::from_str(json.as_str())?;
        Ok(Box::from(BiomeElement::new(name, shared_data, format_data)))
    }

    fn get_file_regex() -> &'static Regex {
        &BIOME_REG
    }
}

//////////////////////////////////////////
//------ Biome Data Serialization ------//
//////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum BiomeFormatData {
    BiomeFormat6(BiomeDataFormat6),
    BiomeFormat8(BiomeDataFormat8),
    BiomeFormat10(BiomeDataFormat10),
    BiomeFormat12(BiomeDataFormat12)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeSharedData {
    temperature: f32,
    #[serde(default)]
    temperature_modifier: TemperatureModifier,
    downfall: f32,
    effects: Effect,
    // TODO: Fix these categories
    //carvers: CarverList,
    //features: FeatureList,
    #[serde(default)]
    creature_spawn_probability: Option<f32>
    // TODO: spawners entry and data
    // TODO: spawn costs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeDataFormat12 {
    has_precipitation: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeDataFormat10 {
    precipitation: LegacyPrecipitationCategory
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeDataFormat8 {
    precipitation: LegacyPrecipitationCategory,
    // TODO: biome category
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeDataFormat6 {
    player_spawn_friendly: bool,
    depth: i32,
    scale: i32,
    precipitation: LegacyPrecipitationCategory,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    //#[serde(default)]
    //particle: Option<Particle>
    // TODO: Rest of spec
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CarverList {
    air: Carver,
    liquid: Carver
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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