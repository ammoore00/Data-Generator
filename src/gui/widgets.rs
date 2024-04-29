use std::fmt::Display;
use iced::{Alignment, Application, Element, Length, settings};
use iced::alignment::{Horizontal, Vertical};
use iced::theme;
use iced::widget::{self, Column, Row, Rule};
use iced_aw::DropDown;
use strum_macros::Display;
use crate::gui::{font, widgets};
use crate::gui::pack_info::DatapackCallbackType;
use crate::gui::window::{ApplicationWindow, Message};

pub(crate) static MAX_CONTENT_WIDTH: f32 = 750.;

pub(crate) static SPACING_SMALL: u16 = 3;
pub(crate) static SPACING_LARGE: u16 = 6;

pub(crate) static STANDARD_RULE_WIDTH: f32 = 4.;

//------------//

#[derive(Debug, Clone)]
pub enum WidgetCallbackChannel {
    PackInfo(DatapackCallbackType)
}

///////////////////////////////
//------ Basic Editors ------//
///////////////////////////////

pub fn create_standard(element: Element<Message>) -> Element<Message> {
    todo!()
}

pub fn create_collapsible(element: Element<Message>) -> Element<Message> {
    todo!()
}

//------------//

pub fn text_editor<'a, F>(
    label: &str,
    default: &str,
    text: &str,
    callback_channel: F
) -> Row<'a, Message, <ApplicationWindow as Application>::Theme>
where F: Fn(String) -> WidgetCallbackChannel + 'a {
    let text = text.replace("\n", "\\n");

    Row::new()
        .push(widget::text(format!("{label}:")))
        .push(widget::text_input(default, &*text)
            .on_input(move |s| {
                Message::Input(callback_channel(s.replace("\\n", "\n")))
            }))
        .align_items(Alignment::Center)
        .spacing(SPACING_LARGE)
}

/////////////////////////////////
//------ Boolean Toggles ------//
/////////////////////////////////

pub fn boolean_toggle<'a, F>(
    label: &str,
    state: bool,
    callback_channel: F
) -> Row<'a, Message, <ApplicationWindow as Application>::Theme>
where F: Fn(bool) -> WidgetCallbackChannel + 'a {
    let mut button_true = widget::button("True");
    let mut button_false = widget::button("False");

    if state {
        button_true = button_true
            .on_press(Message::Input(callback_channel(true)))
            .style(theme::Button::Positive);
        button_false = button_false
            .on_press(Message::Input(callback_channel(false)))
            .style(theme::Button::Secondary);
    }
    else {
        button_true = button_true
            .on_press(Message::Input(callback_channel(true)))
            .style(theme::Button::Secondary);
        button_false = button_false
            .on_press(Message::Input(callback_channel(false)))
            .style(theme::Button::Destructive);
    }

    // TODO: Style

    Row::new()
        .push(widget::text(format!("{label}: ")))
        .push(button_true)
        .push(button_false)
        .align_items(Alignment::Center)
}

pub fn boolean_toggle_optional<'a, F>(
    label: &str,
    state: Option<bool>,
    callback_channel: F
) -> Row<'a, Message, <ApplicationWindow as Application>::Theme>
where F: Fn(Option<bool>) -> WidgetCallbackChannel + 'a {
    let mut button_true = widget::button("True");
    let mut button_false = widget::button("False");

    if let Some(state) = state {
        if state {
            button_true = button_true
                .on_press(Message::Input(callback_channel(None)))
                .style(theme::Button::Positive);
            button_false = button_false
                .on_press(Message::Input(callback_channel(Some(false))))
                .style(theme::Button::Secondary);
        }
        else {
            button_true = button_true
                .on_press(Message::Input(callback_channel(Some(true))))
                .style(theme::Button::Secondary);
            button_false = button_false
                .on_press(Message::Input(callback_channel(None)))
                .style(theme::Button::Destructive);
        }
    }
    else {
        button_true = button_true
            .on_press(Message::Input(callback_channel(Some(true))))
            .style(theme::Button::Secondary);
        button_false = button_false
            .on_press(Message::Input(callback_channel(Some(false))))
            .style(theme::Button::Secondary);

    }

    // TODO: Style

    Row::new()
        .push(widget::text(format!("{label}: ")))
        .push(button_true)
        .push(button_false)
        .align_items(Alignment::Center)
}

///////////////////////////
//------ Dropdowns ------//
///////////////////////////

pub fn dropdown<'a, DropdownType, MessageCallback>(
    label: Option<&str>,
    state: &DropdownState<DropdownType>,
    message_callback: MessageCallback
) -> Element<'a, Message, <ApplicationWindow as Application>::Theme>
where
    DropdownType: for<'b> DropdownOption<'b>,
    MessageCallback: Fn(DropdownEvent<DropdownType>) -> WidgetCallbackChannel
{
    let dropdown_underlay = widget::button(widget::text(format!("{} | v", state.selected)))
        .on_press(Message::Input(message_callback(DropdownEvent::Expand)))
        .style(theme::Button::Secondary);

    let dropdown_overlay = widget::container(Column::with_children(
        DropdownType::variants().iter().map(|variant| {
            widget::button(widget::text(variant.to_string()))
                .on_press(Message::Input(message_callback(DropdownEvent::Select(*variant))))
                .style(theme::Button::Secondary)
                .width(Length::Fill)
                .into()
        })))
        .style(theme::Container::Box);

    let dropdown = DropDown::new(dropdown_underlay, dropdown_overlay, state.expanded)
        .on_dismiss(Message::Input(message_callback(DropdownEvent::Dismiss)));

    if let Some(label) = label {
        Row::new()
            .push(widget::text(label))
            .push(dropdown)
            .align_items(Alignment::Center)
            .spacing(SPACING_SMALL)
            .into()
    }
    else {
        dropdown.into()
    }
}

//------------//

pub fn handle_dropdown_event<T>(
    dropdown_event: DropdownEvent<T>,
    state: &mut DropdownState<T>
)
where T: for<'a> DropdownOption<'a> {
    use DropdownEvent::*;
    match dropdown_event {
        Select(t) => {
            state.selected = t;
            state.expanded = false;
        }
        Dismiss => state.expanded = false,
        Expand => state.expanded = !state.expanded,
    }
}

//------------//

pub trait DropdownOption<'a>: Display + Copy {
    fn variants() -> &'a[Self] where Self: Sized;
}

//------------//

#[derive(Clone, Debug, Display)]
pub enum DropdownEvent<T>
where T: for<'a> DropdownOption<'a> {
    Select(T),
    Expand,
    Dismiss,
}

//------------//

#[derive(Clone, Debug, Default)]
pub struct DropdownState<T>
where T: for<'a> DropdownOption<'a> {
    pub(crate) selected: T,
    pub(crate) expanded: bool,
}

impl<T> DropdownState<T>
where T: for<'a> DropdownOption<'a> {
    pub fn new(t: T) -> Self {
        Self {
            selected: t,
            expanded: false
        }
    }
}

///////////////////////
//------ Lists ------//
///////////////////////

pub fn list<'a, T, EditEventType, InlineWidgetCreator, MessageCallback, ContentState>(
    label: &str,
    data: &Vec<T>,
    list_state: &ListState,
    content_state: &ContentState,
    settings: ListSettings<'a, T, ContentState>,
    inline_widget_creator: InlineWidgetCreator,
    message_callback: MessageCallback
) -> Element<'a, Message, <ApplicationWindow as Application>::Theme>
where
    T: Default,
    InlineWidgetCreator: Fn(&T, usize, bool, &ContentState) -> Option<Element<'a, Message, <ApplicationWindow as Application>::Theme>>,
    MessageCallback: Fn(ListEvent<EditEventType>) -> WidgetCallbackChannel + 'a,
{
    let button_size = 24.;
    let sidebar_padding = 4;

    let name = widget::text(label);
    let add_top = widget::button(" + ")
        .on_press(Message::Input(message_callback(ListEvent::Add(0))))
        .style(theme::Button::Positive);
    let header = Row::new()
        .push(name).push(add_top)
        .align_items(Alignment::Center)
        .spacing(SPACING_LARGE);

    let mut content = Column::new()
        .spacing(SPACING_SMALL);

    for i in 0..data.len() {
        let item = &data[i];
        let collapsed = list_state.is_node_collapsed(i);

        let add_button = widget::button(
            font::icon(font::PLUS_ICON)
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center))
            .on_press(Message::Input(message_callback(ListEvent::Add(i + 1))))
            .style(theme::Button::Positive)
            .height(Length::Fixed(button_size))
            .width(Length::Fixed(button_size))
            .padding(0);
        let remove_button = widget::button(
                font::icon(font::TRASH_ICON)
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center))
            .on_press(Message::Input(message_callback(ListEvent::Remove(i))))
            .style(theme::Button::Destructive)
            .height(Length::Fixed(button_size))
            .width(Length::Fixed(button_size))
            .padding(0);

        let mut add_remove_buttons = Row::new()
            .push(add_button);

        if !settings.required || data.len() > 1 {
            add_remove_buttons = add_remove_buttons.push(remove_button);
        }

        let mut up_button = widget::button(
                widget::text("^")
                    .size(11)
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center))
            .style(theme::Button::Secondary)
            .height(Length::Fixed(button_size * 2. / 3.))
            .width(Length::Fixed(button_size))
            .padding(0);
        let mut down_button = widget::button(
                widget::text("v")
                    .size(9)
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center))
            .style(theme::Button::Secondary)
            .height(Length::Fixed(button_size * 2. / 3.))
            .width(Length::Fixed(button_size))
            .padding(0);

        if i > 0 {
            up_button = up_button.on_press(Message::Input(message_callback(ListEvent::Move(MoveDirection::Up, i))))
        }
        if i < data.len() - 1 {
            down_button = down_button.on_press(Message::Input(message_callback(ListEvent::Move(MoveDirection::Down, i))))
        }

        let move_buttons = Column::new()
            .push(up_button)
            .push(down_button);

        let mut controls = Row::new()
            .push(add_remove_buttons)
            .push(move_buttons)
            .align_items(Alignment::Center)
            .spacing(SPACING_SMALL);

        let inline_widget = inline_widget_creator(item, i, collapsed, content_state);

        let mut should_render_collapse_button = false;

        let mut entry = if let ListInlineState::Extended(extended_widget) = &settings.inline_state {
            if let Some(inline_widget) = inline_widget {
                controls = controls.push(inline_widget);
            }

            let mut entry = Column::new().push(controls);
            if let Some(extended_widget) = extended_widget(item, i, content_state) {
                if !collapsed {
                    entry = entry.push(extended_widget);
                }
                should_render_collapse_button = true;
            }
            entry
        }
        else {
            if let Some(inline_widget) = inline_widget {
                controls = controls.push(inline_widget);
            }
            Column::new()
                .push(controls)
        }
        .spacing(SPACING_LARGE);

        if i < data.len() - 1 {
            entry = entry.push(Rule::horizontal(STANDARD_RULE_WIDTH))
        }

        let entry = if should_render_collapse_button {
            let collapse_text = if collapsed { ">" } else { "v" };

            let collapse_button = widget::button(
                widget::text(collapse_text)
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center))
                .on_press(Message::Input(message_callback(ListEvent::Collapse(i, !list_state.is_node_collapsed(i)))))
                .style(theme::Button::Secondary)
                .height(Length::Fixed(button_size))
                .width(Length::Fixed(button_size))
                .padding(0);

            let collapse_button = widget::container(collapse_button)
                .padding(sidebar_padding);

            Row::new()
                .push(collapse_button)
                .push(entry)
        }
        else {
            Row::new()
                .push(widget::text("")
                    .width(Length::Fixed(button_size + 2. * sidebar_padding as f32)))
                .push(entry)
        };

        content = content.push(entry);
    }

    widget::container(
        Column::new()
            .push(header)
            .push(Rule::horizontal(STANDARD_RULE_WIDTH))
            .push(content)
            .spacing(SPACING_SMALL)
        ).into()
}

pub fn handle_list_event<T, Event>(
    list_event: ListEvent<Event>,
    data: &mut Vec<T>,
    state: &mut ListState) -> ListEvent<Event>
where
    T: Default
{
    use ListEvent::*;
    match list_event {
        Add(index) => {
            data.insert(index, T::default());
            state.add(index);
        }
        Remove(index) => {
            data.remove(index);
            state.remove(index);
        },
        Move(ref direction, index) => {
            use MoveDirection::*;
            match direction {
                Up => if index > 0 {
                    data.swap(index, index - 1);
                    state.swap(index, index - 1);
                },
                Down => if index < data.len() - 1 {
                    data.swap(index, index + 1);
                    state.swap(index, index + 1);
                }
            }
        },
        Collapse(index, is_collapsed) => state.set_collapsed(index, is_collapsed),
        _ => {}
    }

    list_event
}

//------------//

#[derive(Debug, Clone)]
pub struct ListState {
    collapsed: Vec<bool>
}

impl ListState {
    pub fn new(count: usize) -> Self {
        Self {
            collapsed: vec![false; count]
        }
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        self.collapsed.swap(i, j);
    }

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

//------------//

pub struct ListSettings<'a, T, S>
where T: Default {
    pub required: bool,
    pub inline_state: ListInlineState<'a, T, S>
}

impl<'a, T, S> Default for ListSettings<'a, T, S>
where T: Default {
    fn default() -> Self {
        Self {
            required: false,
            inline_state: ListInlineState::Inline
        }
    }
}

//------------//

pub enum ListInlineState<'a, T, S>
where T: Default {
    Inline,
    Extended(Box<dyn Fn(&T, usize, &S) -> Option<Element<'a, Message, <ApplicationWindow as Application>::Theme>>>)
}

//------------//

#[derive(Debug, Clone)]
pub enum ListEvent<T> {
    Add(usize),
    Remove(usize),
    Move(MoveDirection, usize),
    Collapse(usize, bool),
    Edit(T, usize)
}

//------------//

#[derive(Debug, Clone)]
pub enum MoveDirection {
    Up,
    Down
}