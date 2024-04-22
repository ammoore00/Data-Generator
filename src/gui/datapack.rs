use iced::Element;
use crate::data::datapack::Datapack;
use crate::gui::datapack::DatapackCallbackType::DatapackName;
use crate::gui::widgets::{text_editor, WidgetCallbackChannel};
use crate::gui::window::Message;

#[derive(Debug, Clone)]
pub enum DatapackCallbackType {
    DatapackName(String)
}

//------------//

pub fn get_datapack_gui<'a>(datapack: &Datapack) -> Element<'a, Message> {
    let name = text_editor("Name", "Name", &datapack.name, |text| WidgetCallbackChannel::PackInfo(DatapackName(text)));

    name.into()
}

//------------//

pub fn handle_datapack_update(datapack: &mut Datapack, callback_type: DatapackCallbackType) {
    match callback_type {
        DatapackName(name) => datapack.name = name.clone(),
    }
}