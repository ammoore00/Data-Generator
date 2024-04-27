use iced::{Application, Element, widget};
use iced::widget::{Column, container};
use crate::data::datapack::{Datapack, Overlay};
use crate::data::util;
use crate::gui::widgets::{self, ListEvent, ListInlineState, ListSettings, ListState, WidgetCallbackChannel};
use crate::gui::window::{ApplicationWindow, Message};

///////////////////////////////
//------ Message Types ------//
///////////////////////////////

#[derive(Debug, Clone)]
pub enum DatapackCallbackType {
    DatapackName(String),
    Description(ListEvent<TextEditEvent>),
    Overlay(ListEvent<OverlayEditEvent>)
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
pub enum OverlayEditEvent {
    Name(String)
}

//------------//

#[derive(Debug, Clone)]
pub struct PackInfoState {
    pub description_state: ListState,
    pub overlay_state: ListState,
}

impl PackInfoState {
    pub fn new(datapack: &Datapack) -> Self {
        Self {
            description_state: ListState::new(datapack.description().len()),
            overlay_state: ListState::new(datapack.overlays().len()),
        }
    }
}

////////////////////////////////////
//------ Message Processing ------//
////////////////////////////////////

pub fn handle_datapack_update(
    datapack: &mut Datapack,
    callback_type: DatapackCallbackType,
    mut pack_info_state: PackInfoState
) -> PackInfoState {
    use DatapackCallbackType::*;
    match callback_type {
        DatapackName(name) => datapack.set_name(&*name),
        Description(list_event) => widgets::handle_list_event(list_event, datapack.description_mut(), &mut pack_info_state.description_state,
            |data, edit_event, index| {
                let mut text = data.get_mut(index).expect("List edit event should not return values out of range");

                use TextEditEvent::*;
                match edit_event {
                    Text(txt) => {
                        text.text = txt;
                    }
                    Bold(is_bold) => text.is_bold = is_bold,
                    Italic(is_italic) => text.is_italic = is_italic,
                    Underlined(is_underlined) => text.is_underlined = is_underlined,
                    Strikethrough(is_strikethrough) => text.is_strikethrough = is_strikethrough,
                    Obfuscated(is_obfuscated) => text.is_obfuscated = is_obfuscated,
                }
            }),
        Overlay(list_event) => widgets::handle_list_event(list_event, datapack.overlays_mut(), &mut pack_info_state.overlay_state,
            |data, edit_event, index| {
                let mut overlay = data.get_mut(index).expect("List edit event should not return values out of range");

                use OverlayEditEvent::*;
                match edit_event {
                    Name(name) => overlay.name = name,
                }
            }),
    }
    pack_info_state
}

////////////////////////////////
//------ GUI generation ------//
////////////////////////////////

pub fn get_pack_info_gui<'a>(
    datapack: &Datapack,
    pack_info_state: &PackInfoState
) -> Element<'a, Message, <ApplicationWindow as Application>::Theme> {
    let name = widgets::text_editor("Name", "Name", &datapack.name(),
        |s| WidgetCallbackChannel::PackInfo(DatapackCallbackType::DatapackName(s)));

    let description = widgets::list("Description", datapack.description(), &pack_info_state.description_state,
        ListSettings {
            required: true,
            inline_state: ListInlineState::Extended(Box::new(|text, collapsed| {
                let preview = if collapsed { Some(widget::text(&text.text.replace("\n", "\\n"))) } else { None };
                preview.map(|p| {p.into()})
            })),
        },
        get_text_gui,
        |list_event| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(list_event)));

    let overlays = widgets::list("Overlays", datapack.overlays(), &pack_info_state.overlay_state,
        ListSettings {
            required: false,
            inline_state: ListInlineState::Extended(Box::new(|overlay, collapsed| {
                if collapsed { Some(widget::text(&overlay.name).into()) } else { None }
            }))
        },
        get_overlay_gui,
        |list_event| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Overlay(list_event)));

    Column::new()
        .push(name)
        .push(description)
        .push(overlays)
        .spacing(widgets::SPACING_LARGE)
        .into()
}

fn get_text_gui<'a>(
    text: &util::Text,
    index: usize
) -> Element<'a, Message> {
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
    ).into()
}

fn get_overlay_gui<'a>(
    overlay: &Overlay,
    index: usize
) -> Element<'a, Message, <ApplicationWindow as Application>::Theme> {
    let text_editor = widgets::text_editor("Overlay", "Name", &overlay.name,
        move |s| {
            WidgetCallbackChannel::PackInfo(
                DatapackCallbackType::Overlay(
                    ListEvent::Edit(OverlayEditEvent::Name(s), index)
                )
            )
        });

    container(Column::new()
        .push(text_editor)
        .spacing(5)
    ).into()
}