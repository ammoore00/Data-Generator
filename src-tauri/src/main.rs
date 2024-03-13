// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::str::FromStr;
use crate::data::datapack::{Datapack, PackInfo};
use crate::data::elements::element::NamedDataElement;

mod data;

static mut DATAPACK: Option<&Datapack> = None;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_pack_mcmeta() -> PackInfo {
    // This is truly terrible, but only exists for testing - TODO: Remove
    Datapack::from_zip("data/Terralith_1.20_v2.4.11.zip").unwrap().pack_info
}

fn main() {
    //let datapack_1_20_4 = Datapack::from_zip("data/1-20-4.zip").unwrap();

    //println!("{:?}", datapack_1_20_4);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_pack_mcmeta])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
