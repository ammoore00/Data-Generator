use iced::{Element, widget};
use iced::widget::Column;
use crate::data::datapack::Datapack;
use crate::data::util;
use crate::gui::datapack::DatapackCallbackType::{DatapackName, Description};
use crate::gui::widgets::{self, WidgetCallbackChannel};
use crate::gui::window::Message;

#[derive(Debug, Clone)]
pub enum DatapackCallbackType {
    DatapackName(String),
    Description {
        index: u32,
        //event: TextCallbackEvent
    }
}

//------------//

#[derive(Debug, Clone)]
pub enum TextCallbackEvent {
    Add,
    Remove,
    Text(String),
}

//------------//

#[derive(Debug, Clone)]
pub struct PackInfoState {
    pub is_default: bool
}

impl PackInfoState {
    pub fn new(datapack: &Datapack) -> Self {
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
    let description = widgets::list("Description", datapack.description(),
        |text| get_text_gui(text),
        |description| WidgetCallbackChannel::PackInfo(Description {index: 0}));

    Column::new().push(name).push(description).into()
}

fn get_text_gui<'a>(text: &util::Text) -> Element<'a, Message> {
    let text_gui = widgets::text_editor("Text", "Text", &text.text, |s| WidgetCallbackChannel::PackInfo(DatapackName(s))).into();
    text_gui
}