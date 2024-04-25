use iced::{Application, Settings};
use crate::gui::window::ApplicationWindow;

mod data;
mod gui;

fn main() -> iced::Result {
    ApplicationWindow::run(Settings::default())
    //Ok(())
}
