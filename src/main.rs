use iced::{Sandbox, Settings};
use crate::data::datapack::{Datapack, SerializableDatapack};
use crate::gui::window::ApplicationWindow;

mod data;
mod gui;

fn main() -> iced::Result {
    //let datapack = SerializableDatapack::from_zip("data/1-20-4.zip").unwrap();
    let ser_datapack = SerializableDatapack::from_zip("data/Terralith_1.20_v2.4.11.zip").unwrap();
    let datapack = Datapack::try_from(ser_datapack).unwrap();

    println!("{:#?}", datapack);

    //ApplicationWindow::run(Settings::default())
    Ok(())
}
