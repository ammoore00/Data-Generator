use iced::Element;
use crate::data::datapack::Datapack;
use crate::gui::datapack::DatapackCallbackType::DatapackName;
use crate::gui::widgets::{self, WidgetCallbackChannel};
use crate::gui::window::Message;

#[derive(Debug, Clone)]
pub enum DatapackCallbackType {
    DatapackName(String),
}

//------------//

pub fn handle_datapack_update(datapack: &mut Datapack, callback_type: DatapackCallbackType) {
    match callback_type {
        DatapackName(name) => datapack.name = name.clone(),
    }
}

pub fn get_datapack_gui<'a>(datapack: &Datapack) -> Element<'a, Message> {
    let name = widgets::text_editor("Name", "Name", &datapack.name, |s| WidgetCallbackChannel::PackInfo(DatapackName(s)));

    name.into()
}

fn get_text_gui<'a>() -> Element<'a, Message> {
    todo!()
}