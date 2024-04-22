use iced::Element;
use crate::data::datapack::Datapack;
use crate::gui::widgets::text_editor;
use crate::gui::window::Message;

pub fn get_datapack_gui<'a>(datapack: &Datapack) -> Element<'a, Message> {
    let name = text_editor("Name", "Name", &datapack.name);

    name.into()
}