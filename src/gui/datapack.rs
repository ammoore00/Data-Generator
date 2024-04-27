use iced::Element;
use iced::widget::Column;
use crate::data::datapack::Datapack;
use crate::data::util;
use crate::gui::datapack::DatapackCallbackType::{DatapackName, Description};
use crate::gui::widgets::{self, AddLocation, ListEvent, WidgetCallbackChannel};
use crate::gui::window::Message;

///////////////////////////////
//------ Message Types ------//
///////////////////////////////

#[derive(Debug, Clone)]
pub enum DatapackCallbackType {
    DatapackName(String),
    Description(ListEvent<TextEditEvent>)
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

////////////////////////////////////
//------ Message Processing ------//
////////////////////////////////////

pub fn handle_datapack_update(datapack: &mut Datapack, callback_type: DatapackCallbackType) {
    match callback_type {
        DatapackName(name) => datapack.set_name(&*name),
        Description(list_event) => widgets::handle_list_event(list_event, datapack.description_mut(),
            |data, edit_event, index| {
                match edit_event {
                    TextEditEvent::Text(text) => {
                        data.get_mut(index).expect("List edit event should not return values out of range").text = text;
                    }
                }
            }),
    }
}

////////////////////////////////
//------ GUI generation ------//
////////////////////////////////

pub fn get_datapack_gui<'a>(datapack: &Datapack) -> Element<'a, Message> {
    let name = widgets::text_editor("Name", "Name", &datapack.name(), |s| WidgetCallbackChannel::PackInfo(DatapackName(s)));
    let description = widgets::list("Description", datapack.description(), get_text_gui,
        |list_event| WidgetCallbackChannel::PackInfo(Description(list_event)));

    Column::new().push(name).push(description).into()
}

fn get_text_gui<'a>(text: &util::Text, index: usize) -> Element<'a, Message> {
    let text_editor = widgets::text_editor("Text", "Text", &text.text,
        move |s| WidgetCallbackChannel::PackInfo(Description(ListEvent::Edit(TextEditEvent::Text(s), index)))
    ).into();

    text_editor
}

#[derive(Debug, Clone)]
enum TextEditEvent {
    Text(String)
}