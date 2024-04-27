use iced::{Alignment, Application, Element, widget};
use iced::widget::{Column, container, Row};
use iced_aw::DropDown;
use strum_macros::Display;
use crate::data::datapack::{Datapack, Overlay};
use crate::data::util;
use crate::gui::widgets::{self, DropdownEvent, DropdownOption, DropdownState, ListEvent, ListInlineState, ListSettings, ListState, SPACING_LARGE, WidgetCallbackChannel};
use crate::gui::window::{ApplicationWindow, Message};

///////////////////////////////
//------ Message Types ------//
///////////////////////////////

#[derive(Debug, Clone)]
pub enum DatapackCallbackType {
    DatapackName(String),
    Description(DescriptionUpdateType),
    Overlay(ListEvent<OverlayEditEvent>)
}

//------------//

#[derive(Debug, Clone)]
enum DescriptionUpdateType {
    Content(ListEvent<TextEditEvent>),
    Type(usize, DropdownEvent<TextType>),
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

#[derive(Clone, Copy, Debug, Default, Display)]
enum TextType {
    #[default]
    String,
    Object
}

impl<'a> DropdownOption<'a> for TextType {
    fn variants() -> &'a [Self] {
        &TEXT_TYPE_CHOICES[..]
    }
}

const TEXT_TYPE_CHOICES: [TextType; 2] = [TextType::String, TextType::Object];

//------------//

#[derive(Debug, Clone)]
pub enum OverlayEditEvent {
    Name(String)
}

//------------//

#[derive(Debug, Clone)]
pub struct PackInfoState {
    pub description_state: DescriptionState,
    pub overlay_state: ListState,
}

impl PackInfoState {
    pub fn new(datapack: &Datapack) -> Self {
        let size = datapack.description().len();
        let text_type_state = vec![DropdownState {
            selected: TextType::String,
            expanded: false,
        }; size];

        Self {
            description_state: DescriptionState {
                collapsed_state: ListState::new(size),
                text_type_state,
            },
            overlay_state: ListState::new(datapack.overlays().len()),
        }
    }
}

//------------//

#[derive(Debug, Clone)]
pub struct DescriptionState {
    collapsed_state: ListState,
    text_type_state: Vec<DropdownState<TextType>>
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
        Description(event) => match event {
            DescriptionUpdateType::Content(list_event) => {
                widgets::handle_list_event(list_event, datapack.description_mut(), &mut pack_info_state.description_state.collapsed_state,
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
                    })
            },
            DescriptionUpdateType::Type(index, type_event) => {
                let text_type_state = &mut pack_info_state.description_state.text_type_state[index];

                use DropdownEvent::*;
                match type_event {
                    Select(text_type) => {
                        text_type_state.selected = text_type;
                        text_type_state.expanded = false;
                    }
                    Dismiss => text_type_state.expanded = false,
                    Expand => text_type_state.expanded = true,
                }
            }
        },
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

    let description = widgets::list("Description", datapack.description(), &pack_info_state.description_state.collapsed_state, &pack_info_state,
        ListSettings {
            required: true,
            inline_state: ListInlineState::Extended(Box::new(|t, i, s| text_gui_extended(t, i, s))),
        },
        |text, index, collapsed, state| text_header_widget(text, index, collapsed, state),
        |list_event| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionUpdateType::Content(list_event))));

    let overlays = widgets::list("Overlays", datapack.overlays(), &pack_info_state.overlay_state, &pack_info_state,
        ListSettings {
            required: false,
            inline_state: ListInlineState::Extended(Box::new(|o, i, s| get_overlay_gui(o, i, s)))
        },
        |overlay: &Overlay, _, collapsed, _: &&PackInfoState| {
            if collapsed { Some(widget::text(&overlay.name).into()) } else { None }
        },
        |list_event| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Overlay(list_event)));

    Column::new()
        .push(name)
        .push(description)
        .push(overlays)
        .spacing(widgets::SPACING_LARGE)
        .into()
}

fn text_gui_extended<'a>(
    text: &util::Text,
    index: usize,
    pack_info_state: &PackInfoState
) -> Option<Element<'a, Message, <ApplicationWindow as Application>::Theme>> {
    if let TextType::String =  pack_info_state.description_state.text_type_state[index].selected {
        return None;
    }

    let text_editor = widgets::text_editor("Text", "Text", &text.text,
        move |s| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionUpdateType::Content(
            ListEvent::Edit(TextEditEvent::Text(s), index))
        )));

    let bold = widgets::boolean_toggle_optional("Bold", text.is_bold,
        move |is_bold| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionUpdateType::Content(
                ListEvent::Edit(TextEditEvent::Bold(is_bold), index)))));
    let italic = widgets::boolean_toggle_optional("Italic", text.is_italic,
        move |is_italic| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionUpdateType::Content(
            ListEvent::Edit(TextEditEvent::Italic(is_italic), index)))));
    let underlined = widgets::boolean_toggle_optional("Underlined", text.is_underlined,
        move |is_underlined| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionUpdateType::Content(
            ListEvent::Edit(TextEditEvent::Underlined(is_underlined), index)))));
    let strikethrough = widgets::boolean_toggle_optional("Strikethrough", text.is_strikethrough,
        move |is_strikethrough| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionUpdateType::Content(
            ListEvent::Edit(TextEditEvent::Strikethrough(is_strikethrough), index)))));
    let obfuscated = widgets::boolean_toggle_optional("Obfuscated", text.is_obfuscated,
        move |is_obfuscated| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionUpdateType::Content(
            ListEvent::Edit(TextEditEvent::Obfuscated(is_obfuscated), index)))));

    // TODO: color and font

    let container = container(Column::new()
        .push(text_editor)
        .push(bold)
        .push(italic)
        .push(underlined)
        .push(strikethrough)
        .push(obfuscated)
        .spacing(5)
    ).into();

    Some(container)
}

fn text_header_widget<'a>(
    text: &util::Text,
    index: usize,
    collapsed: bool,
    pack_info_state: &PackInfoState
) -> Option<Element<'a, Message, <ApplicationWindow as Application>::Theme>> {
    let dropdown = widgets::dropdown(None, &pack_info_state.description_state.text_type_state[index], |dropdown_event| {
        WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionUpdateType::Type(index, dropdown_event)))
    });

    let widget = if let TextType::String = pack_info_state.description_state.text_type_state[index].selected {
        let text_editor = widget::text_input("Text", &text.text.replace("\n", "\\n"))
            .on_input(move |s| Message::Input(WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionUpdateType::Content(
                ListEvent::Edit(TextEditEvent::Text(s.replace("\\n", "\n")), index))
            ))));

        Row::new()
            .push(dropdown)
            .push(text_editor)
    }
    else {
        let mut header = Row::new()
            .push(dropdown);

        if collapsed {
            header = header.push(widget::text(&text.text.replace("\n", "\\n")))
        }

        header
    }
    .align_items(Alignment::Center)
    .spacing(SPACING_LARGE)
    .into();

    Some(widget)
}

fn get_overlay_gui<'a>(
    overlay: &Overlay,
    index: usize,
    _pack_info_state: &PackInfoState
) -> Option<Element<'a, Message, <ApplicationWindow as Application>::Theme>> {
    let text_editor = widgets::text_editor("Overlay", "Name", &overlay.name,
        move |s| {
            WidgetCallbackChannel::PackInfo(
                DatapackCallbackType::Overlay(
                    ListEvent::Edit(OverlayEditEvent::Name(s), index)
                )
            )
        });

    Some(container(Column::new()
        .push(text_editor)
        .spacing(5)
    ).into())
}