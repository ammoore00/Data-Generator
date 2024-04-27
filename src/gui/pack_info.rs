use iced::{Application, Element, widget};
use iced::widget::{Column, container, Row};
use iced_aw::DropDown;
use strum_macros::Display;
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
    Description(DescriptionUpdateType),
    Overlay(ListEvent<OverlayEditEvent>)
}

//------------//

#[derive(Debug, Clone)]
enum DescriptionUpdateType {
    Content(ListEvent<TextEditEvent>),
    Type(usize, TextTypeEvent),
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
enum TextTypeEvent {
    Select(TextType),
    Dismiss,
    Expand
}

//------------//

#[derive(Clone, Debug, Default, Display)]
enum TextType {
    #[default]
    String,
    Object
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
        let text_type_state = vec![TextTypeState {
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
    text_type_state: Vec<TextTypeState>
}

//------------//

#[derive(Debug, Clone)]
pub struct TextTypeState {
    selected: TextType,
    expanded: bool
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

                use TextTypeEvent::*;
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
            inline_state: ListInlineState::Extended(Box::new(get_text_gui)),
        },
        |text, index, collapsed, state| get_text_header_widget(text, index, collapsed, state),
        |list_event| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionUpdateType::Content(list_event))));

    let overlays = widgets::list("Overlays", datapack.overlays(), &pack_info_state.overlay_state, &pack_info_state,
        ListSettings {
            required: false,
            inline_state: ListInlineState::Extended(Box::new(get_overlay_gui))
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

fn get_text_gui<'a>(
    text: &util::Text,
    index: usize
) -> Element<'a, Message> {
    let text_editor = widgets::text_editor("Text", "Text", &text.text,
        move |s| {
            WidgetCallbackChannel::PackInfo(
                DatapackCallbackType::Description(
                    DescriptionUpdateType::Content(
                        ListEvent::Edit(TextEditEvent::Text(s), index)
                    )
                )
            )
        });

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

fn get_text_header_widget<'a>(
    text: &util::Text,
    index: usize,
    collapsed: bool,
    pack_info_state: &PackInfoState
) -> Option<Element<'a, Message, <ApplicationWindow as Application>::Theme>> {
    let dropdown_underlay: Row<'a, Message, <ApplicationWindow as Application>::Theme> = Row::new()
        .push(widget::button(widget::text(format!("{} | v", pack_info_state.description_state.text_type_state[index].selected)))
            .on_press(Message::Input(WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionUpdateType::Type(index, TextTypeEvent::Expand))))));

    let dropdown_overlay: Column<'a, Message, <ApplicationWindow as Application>::Theme> = Column::with_children(TEXT_TYPE_CHOICES.map(|text_type| {
        Row::new()
            .push(widget::button(widget::text(text_type.to_string()))
                .on_press(Message::Input(WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionUpdateType::Type(index, TextTypeEvent::Select(text_type)))))))
            .into()
    }));

    let dropdown = DropDown::new(dropdown_underlay, dropdown_overlay, pack_info_state.description_state.text_type_state[index].expanded)
        .on_dismiss(Message::Input(WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionUpdateType::Type(index, TextTypeEvent::Dismiss)))));

    let preview = widget::text(&text.text.replace("\n", "\\n"));

    let mut row = Row::new()
        .push(dropdown);

    if collapsed {
        row = row.push(preview);
    }

    Some(row.into())
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