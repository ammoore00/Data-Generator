use iced::{Alignment, Element};
use iced::widget::{Row, text, text_input};
use crate::data::datapack::Datapack;
use crate::gui::window::Message;

pub fn get_datapack_gui<'a>(datapack: &Datapack) -> Element<'a, Message> {
    let name = Row::new()
        .push(text("Name"))
        .push(text_input("Name", &datapack.name)
            .on_input(|text| {
                Message::Input(text)
            }))
        .align_items(Alignment::Center)
        .spacing(10);

    name.into()
}