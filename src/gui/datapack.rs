use iced::Element;
use iced::widget::{Column, container, Row};
use crate::data::datapack::Datapack;
use crate::data::util;
use crate::gui::widgets::{self, ListEvent, ListState, WidgetCallbackChannel};
use crate::gui::window::Message;

///////////////////////////////
//------ Message Types ------//
///////////////////////////////

#[derive(Debug, Clone)]
pub enum DatapackCallbackType {
    DatapackName(String),
    Description(ListEvent<TextEditEvent>),
}

//------------//

#[derive(Debug, Clone)]
enum TextEditEvent {
    Text(String),

    Bold(Option<bool>),
    Italic(Option<bool>),
    Underlined(Option<bool>),
    Strikethrough(Option<bool>),
    Obfuscated(Option<bool>),
}

//------------//

#[derive(Debug, Clone)]
pub struct PackInfoState {
    pub is_default: bool,
    pub description_state: DescriptionState,
}

impl PackInfoState {
    pub fn new(datapack: &Datapack) -> Self {
        Self {
            is_default: true,
            description_state: DescriptionState::new(datapack.description().len()),
        }
    }
}

//------------//

#[derive(Debug, Clone)]
pub struct DescriptionState {
    collapsed: Vec<bool>
}

impl DescriptionState {
    fn new(count: usize) -> Self {
        Self {
            collapsed: vec![false; count]
        }
    }
}

impl ListState for DescriptionState {
    fn is_node_collapsed(&self, index: usize) -> bool {
        *self.collapsed.get(index).unwrap_or(&false)
    }

    fn set_collapsed(&mut self, index: usize, collapsed: bool) {
        if let Some(mut val) = self.collapsed.get_mut(index) {
            *val = collapsed;
        }
    }

    fn add(&mut self, index: usize) {
        self.collapsed.insert(index, false);
    }

    fn remove(&mut self, index: usize) {
        self.collapsed.remove(index);
    }
}

////////////////////////////////////
//------ Message Processing ------//
////////////////////////////////////

pub fn handle_datapack_update(datapack: &mut Datapack, callback_type: DatapackCallbackType, mut pack_info_state: PackInfoState) -> PackInfoState {
    use DatapackCallbackType::*;
    match callback_type {
        DatapackName(name) => datapack.set_name(&*name),
        Description(list_event) => widgets::handle_list_event(list_event, datapack.description_mut(), &mut pack_info_state.description_state,
            |data, edit_event, index| {
                let mut item = data.get_mut(index).expect("List edit event should not return values out of range");

                use TextEditEvent::*;
                match edit_event {
                    Text(text) => {
                        item.text = text;
                    }
                    Bold(is_bold) => item.is_bold = is_bold,
                    Italic(is_italic) => item.is_italic = is_italic,
                    Underlined(is_underlined) => item.is_underlined = is_underlined,
                    Strikethrough(is_strikethrough) => item.is_strikethrough = is_strikethrough,
                    Obfuscated(is_obfuscated) => item.is_obfuscated = is_obfuscated,
                }
            }),
    }
    pack_info_state
}

////////////////////////////////
//------ GUI generation ------//
////////////////////////////////

pub fn get_pack_info_gui<'a>(datapack: &Datapack, pack_info_state: &PackInfoState) -> Element<'a, Message> {
    let name = widgets::text_editor("Name", "Name", &datapack.name(),
        |s| WidgetCallbackChannel::PackInfo(DatapackCallbackType::DatapackName(s)));
    let description = widgets::list("Description", datapack.description(), &pack_info_state.description_state, get_text_gui,
        |list_event| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(list_event)));

    Column::new()
        .push(name)
        .push(description)
        .spacing(widgets::SPACING_SMALL)
        .into()
}

fn get_text_gui<'a>(text: &util::Text, index: usize) -> Element<'a, Message> {
    let text_editor = widgets::text_editor("Text", "Text", &text.text,
        move |s| {
            WidgetCallbackChannel::PackInfo(
                DatapackCallbackType::Description(
                    ListEvent::Edit(TextEditEvent::Text(s), index)
                )
            )
        });

    let bold = widgets::boolean_toggle_optional("Bold", text.is_bold,
        move |is_bold| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(
                ListEvent::Edit(TextEditEvent::Bold(is_bold), index))));
    let italic = widgets::boolean_toggle_optional("Italic", text.is_italic,
        move |is_italic| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(
            ListEvent::Edit(TextEditEvent::Italic(is_italic), index))));
    let underlined = widgets::boolean_toggle_optional("Underlined", text.is_underlined,
        move |is_underlined| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(
            ListEvent::Edit(TextEditEvent::Underlined(is_underlined), index))));
    let strikethrough = widgets::boolean_toggle_optional("Strikethrough", text.is_strikethrough,
        move |is_strikethrough| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(
            ListEvent::Edit(TextEditEvent::Strikethrough(is_strikethrough), index))));
    let obfuscated = widgets::boolean_toggle_optional("Obfuscated", text.is_obfuscated,
        move |is_obfuscated| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(
            ListEvent::Edit(TextEditEvent::Obfuscated(is_obfuscated), index))));

    // TODO: color and font

    container(Column::new()
        .push(text_editor)
        .push(bold)
        .push(italic)
        .push(underlined)
        .push(strikethrough)
        .push(obfuscated)
        .spacing(5)
    )
    .into()
}