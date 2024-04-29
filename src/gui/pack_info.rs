use std::cmp::min;
use iced::{Alignment, Application, Element, Font, widget};
use iced::widget::{Column, container, Row, Rule};
use strum_macros::Display;
use crate::data::datapack::{Datapack, DatapackFormat, Overlay};
use crate::data::{datapack, util};
use crate::gui::widgets::{self, DropdownEvent, DropdownOption, DropdownState, ListEvent, ListInlineState, ListSettings, ListState, SPACING_LARGE, WidgetCallbackChannel};
use crate::gui::widgets::MoveDirection::{Down, Up};
use crate::gui::window;
use crate::gui::window::{ApplicationWindow, Message};

////////////////////////////////////
//------ Message Processing ------//
////////////////////////////////////

#[derive(Debug, Clone)]
pub enum DatapackCallbackType {
    DatapackName(String),
    Description(DescriptionEvent),
    Format(FormatEvent),
    Overlay(ListEvent<OverlayEditEvent>)
}

//------------//

pub fn handle_datapack_update(
    datapack: &mut Datapack,
    callback_type: DatapackCallbackType,
    mut pack_info_state: PackInfoState
) -> PackInfoState {
    use DatapackCallbackType::*;
    match callback_type {
        DatapackName(name) => datapack.set_name(&*name),
        Description(event) => match event {
            DescriptionEvent::Content(list_event) => {
                let list_event = widgets::handle_list_event(list_event, datapack.description_mut(), &mut pack_info_state.description_state.collapsed_state);

                use ListEvent::*;
                match list_event {
                    Add(index) => pack_info_state.description_state.text_type_state.insert(index, DropdownState::default()),
                    Remove(index) => { pack_info_state.description_state.text_type_state.remove(index); },
                    Move(direction, index) => {
                        use crate::gui::widgets::MoveDirection::*;
                        match direction {
                            Up => if index > 0 {
                                pack_info_state.description_state.text_type_state.swap(index, index - 1);
                            },
                            Down => if index < pack_info_state.description_state.text_type_state.len() - 1 {
                                pack_info_state.description_state.text_type_state.swap(index, index + 1);
                            }
                        }
                    }
                    Edit(edit_event, index) => {
                        let mut text = datapack.description_mut().get_mut(index).expect("List edit event should not return values out of range");

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
                    }
                    _ => {}
                }
            },
            DescriptionEvent::Type(index, type_event) => {
                widgets::handle_dropdown_event(type_event, &mut pack_info_state.description_state.text_type_state[index]);
            }
        },
        Format(format_event) => {
            use FormatEvent::*;
            match format_event {
                Type(event) => widgets::handle_dropdown_event(event, &mut pack_info_state.format_state.format_type),
                Format(index, event) => {
                    let state = match index {
                        0 => &mut pack_info_state.format_state.root,
                        1 => &mut pack_info_state.format_state.min,
                        _ => &mut pack_info_state.format_state.max,
                    };
                    widgets::handle_dropdown_event(event, state);
                }
            }
        }
        Overlay(list_event) => {
            let list_event = widgets::handle_list_event(list_event, datapack.overlays_mut(), &mut pack_info_state.overlay_state.list_state);

            use ListEvent::*;
            match list_event {
                Edit(edit_event, index) => {
                    let mut overlay = datapack.overlays_mut().get_mut(index).expect("List edit event should not return values out of range");

                    use OverlayEditEvent::*;
                    match edit_event {
                        Name(name) => overlay.name = name,
                        Format(format_event) => {
                            use FormatEvent::*;
                            match format_event {
                                Type(event) => widgets::handle_dropdown_event(event, &mut pack_info_state.overlay_state.overlay_format_states[index].format_type),
                                Format(format_number, event) => {
                                    let state = match format_number {
                                        0 => &mut pack_info_state.overlay_state.overlay_format_states[index].min,
                                        _ => &mut pack_info_state.overlay_state.overlay_format_states[index].max,
                                    };
                                    widgets::handle_dropdown_event(event, state);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        },
    }
    pack_info_state
}

////////////////////////////////
//------ GUI generation ------//
////////////////////////////////

#[derive(Debug, Clone)]
pub struct PackInfoState {
    description_state: DescriptionState,
    format_state: DatapackFormatState,
    overlay_state: OverlayState,
}

impl PackInfoState {
    pub fn new(datapack: &Datapack) -> Self {
        let size = datapack.description().len();
        let text_type_state = vec![DropdownState::default(); size];

        Self {
            description_state: DescriptionState {
                collapsed_state: ListState::new(size),
                text_type_state,
            },
            format_state: DatapackFormatState::new(datapack),
            overlay_state: OverlayState::new(datapack),
        }
    }
}

pub fn pack_info_gui<'a>(
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
        |list_event| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionEvent::Content(list_event))));

    let format = format_selector(pack_info_state);

    let overlays = widgets::list("Overlays", datapack.overlays(), &pack_info_state.overlay_state.list_state, &pack_info_state,
        ListSettings {
            required: false,
            inline_state: ListInlineState::Extended(Box::new(|o, i, s| overlay_gui(o, i, s)))
        },
        |overlay: &Overlay, _, collapsed, _: &&PackInfoState| {
            if collapsed { Some(widget::text(&overlay.name).into()) } else { None }
        },
        |list_event| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Overlay(list_event)));

    let mut widget = Column::new()
        .push(widget::container(name).max_width(widgets::MAX_CONTENT_WIDTH))
        .push(Rule::horizontal(widgets::STANDARD_RULE_WIDTH))
        .push(widget::container(description).max_width(widgets::MAX_CONTENT_WIDTH))
        .push(Rule::horizontal(widgets::STANDARD_RULE_WIDTH))
        .push(widget::container(format).max_width(widgets::MAX_CONTENT_WIDTH))
        .push(Rule::horizontal(widgets::STANDARD_RULE_WIDTH))
        .spacing(SPACING_LARGE);

    if !is_single_format(pack_info_state) {
        widget = widget.push(widget::container(overlays).max_width(widgets::MAX_CONTENT_WIDTH));
    }

    widget::container(widget)
        .into()
}

//------ Description ------//

fn text_gui_extended<'a>(
    text: &util::Text,
    index: usize,
    pack_info_state: &PackInfoState
) -> Option<Element<'a, Message, <ApplicationWindow as Application>::Theme>> {
    if let TextType::String =  pack_info_state.description_state.text_type_state[index].selected {
        return None;
    }

    let text_editor = Row::new()
        .push(widget::text("Text:"))
        .push(widget::text_input("Text", &*text.text.replace("\n", "\\n"))
            .on_input(move |s| {
                Message::Input(WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionEvent::Content(
                    ListEvent::Edit(TextEditEvent::Text(s.replace("\\n", "\n")), index)))))
            })
            .font(Font::with_name(window::MINECRAFT_FONT)))
        .align_items(Alignment::Center)
        .spacing(SPACING_LARGE);

    let bold = widgets::boolean_toggle_optional("Bold", text.is_bold,
        move |is_bold| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionEvent::Content(
                ListEvent::Edit(TextEditEvent::Bold(is_bold), index)))));
    let italic = widgets::boolean_toggle_optional("Italic", text.is_italic,
        move |is_italic| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionEvent::Content(
            ListEvent::Edit(TextEditEvent::Italic(is_italic), index)))));
    let underlined = widgets::boolean_toggle_optional("Underlined", text.is_underlined,
        move |is_underlined| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionEvent::Content(
            ListEvent::Edit(TextEditEvent::Underlined(is_underlined), index)))));
    let strikethrough = widgets::boolean_toggle_optional("Strikethrough", text.is_strikethrough,
        move |is_strikethrough| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionEvent::Content(
            ListEvent::Edit(TextEditEvent::Strikethrough(is_strikethrough), index)))));
    let obfuscated = widgets::boolean_toggle_optional("Obfuscated", text.is_obfuscated,
        move |is_obfuscated| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionEvent::Content(
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
    )
    .into();

    Some(container)
}

fn text_header_widget<'a>(
    text: &util::Text,
    index: usize,
    collapsed: bool,
    pack_info_state: &PackInfoState
) -> Option<Element<'a, Message, <ApplicationWindow as Application>::Theme>> {
    let dropdown = widgets::dropdown(None, &pack_info_state.description_state.text_type_state[index], |dropdown_event| {
        WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionEvent::Type(index, dropdown_event)))
    });

    let widget = if let TextType::String = pack_info_state.description_state.text_type_state[index].selected {
        let text_editor = widget::text_input("Text", &text.text.replace("\n", "\\n"))
            .on_input(move |s| Message::Input(WidgetCallbackChannel::PackInfo(DatapackCallbackType::Description(DescriptionEvent::Content(
                ListEvent::Edit(TextEditEvent::Text(s.replace("\\n", "\n")), index))
            ))));

        widget::container(Row::new()
                .push(dropdown)
                .push(text_editor)
                .align_items(Alignment::Center)
                .spacing(SPACING_LARGE))
    }
    else {
        let mut header = Row::new()
            .push(dropdown);

        if collapsed {
            header = header.push(widget::text(&text.text.replace("\n", "\\n")))
        }

        widget::container(header
            .align_items(Alignment::Center)
            .spacing(SPACING_LARGE))
    }
    .into();

    Some(widget)
}

#[derive(Debug, Clone)]
pub enum DescriptionEvent {
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
pub struct DescriptionState {
    collapsed_state: ListState,
    text_type_state: Vec<DropdownState<TextType>>
}

//------ Datapack Formats ------//

fn format_selector<'a>(
    pack_info_state: &PackInfoState
) -> Element<'a, Message, <ApplicationWindow as Application>::Theme> {
    let label = widget::text("Format:");

    let format_type = widgets::dropdown(None, &pack_info_state.format_state.format_type,
        |event| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Format(FormatEvent::Type(event))));
    let root_format = widgets::dropdown(None, &pack_info_state.format_state.root,
        |event| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Format(FormatEvent::Format(0, event))));
    let min_format = widgets::dropdown(None, &pack_info_state.format_state.min,
        |event| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Format(FormatEvent::Format(1, event))));
    let max_format = widgets::dropdown(None, &pack_info_state.format_state.max,
        |event| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Format(FormatEvent::Format(2, event))));

    let mut format_selectors = Column::new()
        .push(root_format)
        .align_items(Alignment::Start)
        .spacing(SPACING_LARGE);

    let label_height = 28;
    let label_padding = 5;

    let root_version_label = if is_single_format(pack_info_state) { "Version:" } else { "Root Version:" };

    let mut format_labels = Column::new()
        .push(widget::text(root_version_label)
            .height(label_height))
        .align_items(Alignment::Start)
        .spacing(SPACING_LARGE)
        .padding(label_padding);

    if !is_single_format(pack_info_state) {
        format_selectors = format_selectors
            .push(min_format)
            .push(max_format);

        format_labels = format_labels
            .push(widget::text("Minimum Version:")
                .height(label_height))
            .push(widget::text("Maximum Version:")
                .height(label_height))
    }

    let widget = Row::new()
        .push(label)
        .push(format_type)
        .align_items(Alignment::Center)
        .spacing(SPACING_LARGE);

    let widget = Row::new()
        .push(widget)
        .push(format_labels)
        .push(format_selectors)
        .align_items(Alignment::Start)
        .spacing(SPACING_LARGE);

    widget.into()
}

fn is_single_format(pack_info_state: &PackInfoState) -> bool {
    pack_info_state.format_state.format_type.selected == FormatType::Single
}

//------------//

#[derive(Copy, Clone, Debug, Display, Default, Eq, PartialEq)]
enum FormatType {
    #[default]
    Single,
    Range
}

impl<'a> DropdownOption<'a> for FormatType {
    fn variants() -> &'a [Self] {
        &FORMAT_TYPES[..]
    }
}

const FORMAT_TYPES: [FormatType; 2] = [
    FormatType::Single,
    FormatType::Range,
];

//------------//

impl<'a> DropdownOption<'a> for DatapackFormat {
    fn variants() -> &'a [Self] {
        &datapack::DATAPACK_FORMATS[..]
    }
}

//------------//

#[derive(Clone, Debug)]
pub enum FormatEvent {
    Type(DropdownEvent<FormatType>),
    Format(usize, DropdownEvent<DatapackFormat>)
}

//------------//

#[derive(Clone, Debug, Default)]
struct DatapackFormatState {
    format_type: DropdownState<FormatType>,
    root: DropdownState<DatapackFormat>,
    min: DropdownState<DatapackFormat>,
    max: DropdownState<DatapackFormat>,
}

impl DatapackFormatState {
    fn new(datapack: &Datapack) -> Self {
        let root = datapack.root_format();
        let min = datapack.min_format();
        let max = datapack.max_format();

        let format_type = if min == max { FormatType::Single } else { FormatType::Range };

        Self {
            format_type: DropdownState::new(format_type),
            root: DropdownState::new(root),
            min: DropdownState::new(min),
            max: DropdownState::new(max),
        }
    }
}

//------ Overlays ------//

fn overlay_gui<'a>(
    overlay: &Overlay,
    index: usize,
    pack_info_state: &PackInfoState
) -> Option<Element<'a, Message, <ApplicationWindow as Application>::Theme>> {
    let text_editor = widgets::text_editor("Overlay Directory", "Directory", &overlay.name,
        move |s| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Overlay(
            ListEvent::Edit(OverlayEditEvent::Name(s), index)
        )));

    let format_type = widgets::dropdown(Some("Format: "), &pack_info_state.overlay_state.overlay_format_states[index].format_type,
        |event| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Overlay(
            ListEvent::Edit(OverlayEditEvent::Format(FormatEvent::Type(event)),index)
        )));
    let min_format = widgets::dropdown(Some("Minimum Version: "), &pack_info_state.overlay_state.overlay_format_states[index].min,
        |event| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Overlay(
            ListEvent::Edit(OverlayEditEvent::Format(FormatEvent::Format(0, event)), index)
        )));
    let max_format = widgets::dropdown(Some("Maximum Version: "), &pack_info_state.overlay_state.overlay_format_states[index].max,
        |event| WidgetCallbackChannel::PackInfo(DatapackCallbackType::Overlay(
            ListEvent::Edit(OverlayEditEvent::Format(FormatEvent::Format(1, event)), index)
        )));

    let formats = Row::new()
        .push(format_type)
        .push(min_format)
        .push(max_format)
        .spacing(SPACING_LARGE);

    Some(container(Column::new()
        .push(text_editor)
        .push(formats)
        .spacing(5)
    ).into())
}

//------------//

#[derive(Clone, Debug)]
pub enum OverlayEditEvent {
    Name(String),
    Format(FormatEvent)
}

//------------//

#[derive(Clone, Debug)]
struct OverlayState {
    list_state: ListState,
    overlay_format_states: Vec<OverlayFormatState>,
}

impl OverlayState {
    fn new(datapack: &Datapack) -> Self {
        let overlays = datapack.overlays();
        let overlay_format_states = overlays.iter().map(|overlay| OverlayFormatState::new(overlay)).collect();

        Self {
            list_state: ListState::new(overlays.len()),
            overlay_format_states
        }
    }
}

//------------//

#[derive(Clone, Debug)]
struct OverlayFormatState {
    format_type: DropdownState<FormatType>,
    min: DropdownState<DatapackFormat>,
    max: DropdownState<DatapackFormat>,
}

impl OverlayFormatState {
    fn new(overlay: &Overlay) -> Self {
        let min = overlay.min_format;
        let max = overlay.max_format;

        let format_type = if min == max { FormatType::Single } else { FormatType::Range };

        Self {
            format_type: DropdownState::new(format_type),
            min: DropdownState::new(min),
            max: DropdownState::new(max),
        }
    }
}