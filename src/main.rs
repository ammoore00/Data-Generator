use iced::{Application, Settings};
use crate::gui::window::ApplicationWindow;

mod data;
mod gui;

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.default_text_size = 14.into();

    ApplicationWindow::run(settings)
    //Ok(())
}
