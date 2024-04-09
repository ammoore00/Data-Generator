use iced::{Sandbox, Settings};
use crate::data::datapack::SerializableDatapack;
use crate::gui::window::ApplicationWindow;

mod data;
mod gui;

fn main() -> iced::Result {
    //let datapack_1_20_4 = Datapack::from_zip("data/1-20-4.zip").unwrap();
    let datapack_terralith = SerializableDatapack::from_zip("data/Terralith_1.20_v2.4.11.zip").unwrap();

    println!("{:?}", datapack_terralith);

    //ApplicationWindow::run(Settings::default())
    Ok(())
}
