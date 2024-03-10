// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::str::FromStr;
use crate::data::datapack::Datapack;
use crate::data::elements::element::NamedDataElement;

mod data;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let datapack_1_20_4 = Datapack::from_zip("data/1-20-4.zip").unwrap();
    let datapack_terralith = Datapack::from_zip("data/Terralith_1.20_v2.4.11.zip").unwrap();

    println!("{:?}", datapack_terralith);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
