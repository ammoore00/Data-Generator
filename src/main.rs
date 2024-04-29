#![allow(unused)]

use std::borrow::Cow;
use iced::{Application, Font, Settings};
use crate::gui::window::ApplicationWindow;

mod data;
mod gui;

fn main() -> iced::Result {
    let settings = Settings {
        fonts: vec![
            Cow::Borrowed(include_bytes!("../resources/assets/font/icons.ttf").as_slice())
        ],
        default_text_size: 14.into(),
        .. Settings::default()
    };

    ApplicationWindow::run(settings)
}
