use iced::Element;
use crate::data::datapack::Datapack;
use crate::data::util;
use crate::gui::datapack::DatapackCallbackType::DatapackName;
use crate::gui::widgets::{self, WidgetCallbackChannel};
use crate::gui::window::Message;

#[derive(Debug, Clone)]
pub enum DatapackCallbackType {
    DatapackName(String),
    Description {
        index: u32,
        event: TextCallbackEvent
    }
}

//------------//

#[derive(Debug, Clone)]
pub enum TextCallbackEvent {
    Add,
    Remove,
    // TODO drag and drop
    Text(String),
}

//------------//

#[derive(Debug, Clone)]
pub struct PackInfoState {
    pub is_default: bool
}

impl Default for PackInfoState {
    fn default() -> Self {
        Self {
            is_default: true
        }
    }
}

//------------//

pub fn handle_datapack_update(datapack: &mut Datapack, callback_type: DatapackCallbackType) {
    match callback_type {
        DatapackName(name) => datapack.set_name(&*name),
        DatapackCallbackType::Description { .. } => {

        }
    }
}

pub fn get_datapack_gui<'a>(datapack: &Datapack) -> Element<'a, Message> {
    let name = widgets::text_editor("Name", "Name", &datapack.name(), |s| WidgetCallbackChannel::PackInfo(DatapackName(s)));

    name.into()
}

fn get_text_gui<'a>(text: &util::Text) -> Element<'a, Message> {
    //let text_gui = widgets::text_editor("Text", "Text", &text.text, |s| WidgetCallbackChannel::PackInfo(DatapackName(s))).into()
    todo!()
}