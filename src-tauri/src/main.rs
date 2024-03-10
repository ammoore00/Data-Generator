// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::str::FromStr;
use crate::data::datapack::DatapackFormat;
use crate::data::elements::biome::BiomeElement;
use crate::data::elements::element::NamedDataElement;
use crate::data::util::ResourceLocation;
use crate::io::json_io::{read_file_as_string, read_file_from_zip};

mod data;
mod io;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let plains_biome: String = read_file_from_zip("data/1-20-4.zip", "data/minecraft/worldgen/biome/plains.json").unwrap();
    let plains: BiomeElement = *BiomeElement::deserialize(ResourceLocation::from_str("minecraft:plains").expect("Resource location error"), DatapackFormat::FORMAT26, plains_biome).expect("Deserialization error");

    println!("{:?}", plains);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
