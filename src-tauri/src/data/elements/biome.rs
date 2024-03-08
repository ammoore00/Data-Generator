use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
use std::default::Default;
use crate::data::datapack::DatapackFormat;
use crate::data::elements::carver::CarverData;
use crate::data::elements::element::Element;
use crate::data::util::{BlockState, ItemNBT, ResourceLocation};

#[derive(Debug)]
pub struct BiomeElement {

}

impl BiomeElement {

}

impl Element for BiomeElement {
    fn serialize(format: DatapackFormat) -> &'static str {
        use DatapackFormat::*;
        match format {
            FORMAT6 | FORMAT7 => {}
            FORMAT8 | FORMAT9 => {}
            FORMAT10 => {}
            FORMAT12 | FORMAT15 | FORMAT18 | FORMAT26 => {}
        }
        todo!()
    }
}

////////////////////////////////////
//------ Biome Data Storage ------//
////////////////////////////////////

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
    precipitation: Precipitation,
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
    precipitation: Precipitation,
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
    precipitation: Precipitation,
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

/////////////////////////////////////////
//------ Biome Data Helper Types ------//
/////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize, EnumString)]
pub enum TemperatureModifier {
    #[strum(default)]
    #[strum(to_string = "translate.biome.temperature_modifier.none")]
    #[serde(alias = "none")]
    None,
    #[strum(to_string = "translate.biome.temperature_modifier.frozen")]
    #[serde(alias = "frozen")]
    Frozen
}
impl Default for TemperatureModifier { fn default() -> Self { TemperatureModifier::None } }

#[derive(Debug, Serialize, Deserialize, EnumString)]
pub enum GrassColorModifier {
    #[strum(default)]
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
#[derive(Debug, Serialize, Deserialize, EnumString)]
pub enum Precipitation {
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
impl Default for Precipitation { fn default() -> Self { Precipitation::None } }

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
enum Particle {
    #[serde(rename = "block")]
    Block(BlockState),
    #[serde(rename = "block_marker")]
    BlockMarker(BlockState),
    #[serde(rename = "falling_dust")]
    FallingDust(BlockState),
    #[serde(rename = "item")]
    Item {
        id: ResourceLocation,
        #[serde(rename = "Count")]
        count: i32,
        tag: ItemNBT
    },
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
struct CarverList {
    air: Carver,
    liquid: Carver
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Carver {
    ID(ResourceLocation),
    CARVER(CarverData),
    LIST(Vec<CarverData>)
}

#[derive(Debug, Serialize, Deserialize)]
struct FeatureList {
    // TODO: Implementation of the 11 steps - needs custom serialization
    // Feature placement has 11 steps (more info on wiki) in order
    // Feature placement must also be in the same order across all biomes
    //      per step so that'll be difficult to ensure
}