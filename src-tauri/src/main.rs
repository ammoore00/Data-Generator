// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::str::FromStr;
use serde_json::json;
use crate::data::datapack::DatapackFormat;
use crate::data::elements::biome::BiomeElement;
use crate::data::elements::element::NamedDataElement;
use crate::data::util::ResourceLocation;

mod data;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
  let test_biome = json!({
      "carvers": {
        "air": [
          "minecraft:cave",
          "minecraft:cave_extra_underground",
          "minecraft:canyon"
        ]
      },
      "downfall": 0.4,
      "effects": {
        "fog_color": 12638463,
        "mood_sound": {
          "block_search_extent": 8,
          "offset": 2.0,
          "sound": "minecraft:ambient.cave",
          "tick_delay": 6000
        },
        "sky_color": 7907327,
        "water_color": 4159204,
        "water_fog_color": 329011
      },
      "features": [
        [],
        [
          "minecraft:lake_lava_underground",
          "minecraft:lake_lava_surface"
        ],
        [
          "minecraft:amethyst_geode"
        ],
        [
          "minecraft:monster_room",
          "minecraft:monster_room_deep"
        ],
        [],
        [],
        [
          "minecraft:ore_dirt",
          "minecraft:ore_gravel",
          "minecraft:ore_granite_upper",
          "minecraft:ore_granite_lower",
          "minecraft:ore_diorite_upper",
          "minecraft:ore_diorite_lower",
          "minecraft:ore_andesite_upper",
          "minecraft:ore_andesite_lower",
          "minecraft:ore_tuff",
          "minecraft:ore_coal_upper",
          "minecraft:ore_coal_lower",
          "minecraft:ore_iron_upper",
          "minecraft:ore_iron_middle",
          "minecraft:ore_iron_small",
          "minecraft:ore_gold",
          "minecraft:ore_gold_lower",
          "minecraft:ore_redstone",
          "minecraft:ore_redstone_lower",
          "minecraft:ore_diamond",
          "minecraft:ore_diamond_medium",
          "minecraft:ore_diamond_large",
          "minecraft:ore_diamond_buried",
          "minecraft:ore_lapis",
          "minecraft:ore_lapis_buried",
          "minecraft:ore_copper",
          "minecraft:underwater_magma",
          "minecraft:disk_sand",
          "minecraft:disk_clay",
          "minecraft:disk_gravel"
        ],
        [],
        [
          "minecraft:spring_water",
          "minecraft:spring_lava"
        ],
        [
          "minecraft:glow_lichen",
          "minecraft:patch_tall_grass_2",
          "minecraft:trees_plains",
          "minecraft:flower_plains",
          "minecraft:patch_grass_plain",
          "minecraft:brown_mushroom_normal",
          "minecraft:red_mushroom_normal",
          "minecraft:patch_sugar_cane",
          "minecraft:patch_pumpkin"
        ],
        [
          "minecraft:freeze_top_layer"
        ]
      ],
      "has_precipitation": true,
      "spawn_costs": {},
      "spawners": {
        "ambient": [
          {
            "type": "minecraft:bat",
            "maxCount": 8,
            "minCount": 8,
            "weight": 10
          }
        ],
        "axolotls": [],
        "creature": [
          {
            "type": "minecraft:sheep",
            "maxCount": 4,
            "minCount": 4,
            "weight": 12
          },
          {
            "type": "minecraft:pig",
            "maxCount": 4,
            "minCount": 4,
            "weight": 10
          },
          {
            "type": "minecraft:chicken",
            "maxCount": 4,
            "minCount": 4,
            "weight": 10
          },
          {
            "type": "minecraft:cow",
            "maxCount": 4,
            "minCount": 4,
            "weight": 8
          },
          {
            "type": "minecraft:horse",
            "maxCount": 6,
            "minCount": 2,
            "weight": 5
          },
          {
            "type": "minecraft:donkey",
            "maxCount": 3,
            "minCount": 1,
            "weight": 1
          }
        ],
        "misc": [],
        "monster": [
          {
            "type": "minecraft:spider",
            "maxCount": 4,
            "minCount": 4,
            "weight": 100
          },
          {
            "type": "minecraft:zombie",
            "maxCount": 4,
            "minCount": 4,
            "weight": 95
          },
          {
            "type": "minecraft:zombie_villager",
            "maxCount": 1,
            "minCount": 1,
            "weight": 5
          },
          {
            "type": "minecraft:skeleton",
            "maxCount": 4,
            "minCount": 4,
            "weight": 100
          },
          {
            "type": "minecraft:creeper",
            "maxCount": 4,
            "minCount": 4,
            "weight": 100
          },
          {
            "type": "minecraft:slime",
            "maxCount": 4,
            "minCount": 4,
            "weight": 100
          },
          {
            "type": "minecraft:enderman",
            "maxCount": 4,
            "minCount": 1,
            "weight": 10
          },
          {
            "type": "minecraft:witch",
            "maxCount": 1,
            "minCount": 1,
            "weight": 5
          }
        ],
        "underground_water_creature": [
          {
            "type": "minecraft:glow_squid",
            "maxCount": 6,
            "minCount": 4,
            "weight": 10
          }
        ],
        "water_ambient": [],
        "water_creature": []
      },
      "temperature": 0.8
    });

  let plains: BiomeElement = *BiomeElement::deserialize(ResourceLocation::from_str("minecraft:plains").expect("Resource location error"), DatapackFormat::FORMAT26, test_biome.to_string()).expect("Deserialization error");

  println!("{:?}", plains);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
