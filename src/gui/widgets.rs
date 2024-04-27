use iced::{Alignment, Application, Element, Length};
use iced::alignment::{Horizontal, Vertical};
use iced::theme;
use iced::widget::{self, Column, Row, Rule};
use crate::gui::datapack::DatapackCallbackType;
use crate::gui::window::{ApplicationWindow, Message};

pub(crate) static SPACING_SMALL: u16 = 3;
pub(crate) static SPACING_LARGE: u16 = 6;

pub(crate) static STANDARD_RULE_WIDTH: f32 = 4.;

//------------//

#[derive(Debug, Clone)]
pub enum WidgetCallbackChannel {
    PackInfo(DatapackCallbackType)
}

//------------//

#[derive(Debug, Clone)]
pub enum ListEvent<T> {
    Add(AddLocation),
    Remove(usize),
    Move(MoveDirection, usize),
    Collapse(usize, bool),
    Edit(T, usize)
}

//------------//

#[derive(Debug, Clone)]
pub enum AddLocation {
    Top,
    Bottom,
    Middle(usize)
}

//------------//

#[derive(Debug, Clone)]
pub enum MoveDirection {
    Up,
    Down
}

//------------//

pub fn text_editor<'a, F>(
    label: &str,
    default: &str,
    reference: &str,
    callback_channel: F
) -> Row<'a, Message, <ApplicationWindow as Application>::Theme>
where F: Fn(String) -> WidgetCallbackChannel + 'a {
    Row::new()
        .push(widget::text(format!("{label}:")))
        .push(widget::text_input(default, reference)
            .on_input(move |text| {
                Message::Input(callback_channel(text))
            }))
        .align_items(Alignment::Center)
        .spacing(SPACING_LARGE)
}

//------------//

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

//------------//

pub fn list<'a, DataType, EditEventType, State, WidgetCreator, MessageCallback>(
    label: &str,
    data: &Vec<DataType>,
    state: &State,
    widget_creator: WidgetCreator,
    callback_channel: MessageCallback
) -> Element<'a, Message, <ApplicationWindow as Application>::Theme>
where
    DataType: Default,
    State: ListState,
    WidgetCreator: Fn(&DataType, usize) -> Element<'a, Message, <ApplicationWindow as Application>::Theme>,
    MessageCallback: Fn(ListEvent<EditEventType>) -> WidgetCallbackChannel + 'a,
{
    let button_size = 24.;
    let sidebar_padding = 4;

    let name = widget::text(label);
    let add_top = widget::button(" + ")
        .on_press(Message::Input(callback_channel(ListEvent::Add(AddLocation::Top))))
        .style(theme::Button::Positive);
    let mut header = Row::new()
        .push(name)
        .align_items(Alignment::Center)
        .spacing(SPACING_LARGE);

    if data.len() == 0 {
        header = header.push(add_top)
    }

    let mut content = Column::new()
        .spacing(SPACING_SMALL);

    for i in 0..data.len() {
        let item = &data[i];

        let add_button = widget::button(
                widget::text("+")
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center))
            .on_press(Message::Input(callback_channel(ListEvent::Add(AddLocation::Middle(i)))))
            .style(theme::Button::Positive)
            .height(Length::Fixed(button_size))
            .width(Length::Fixed(button_size))
            .padding(0);
        let remove_button = widget::button(
                widget::text("-")
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center))
            .on_press(Message::Input(callback_channel(ListEvent::Remove(i))))
            .style(theme::Button::Destructive)
            .height(Length::Fixed(button_size))
            .width(Length::Fixed(button_size))
            .padding(0);

        let add_remove_buttons = Row::new()
            .push(add_button)
            .push(remove_button);

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
            up_button = up_button.on_press(Message::Input(callback_channel(ListEvent::Move(MoveDirection::Up, i))))
        }
        if i < data.len() - 1 {
            down_button = down_button.on_press(Message::Input(callback_channel(ListEvent::Move(MoveDirection::Down, i))))
        }

        let move_buttons = Column::new()
            .push(up_button)
            .push(down_button);

        let controls = Row::new()
            .push(add_remove_buttons)
            .push(move_buttons)
            .align_items(Alignment::Center)
            .spacing(SPACING_SMALL);

        let mut entry = Column::new()
            .push(controls)
            .spacing(SPACING_SMALL);

        if !state.is_node_collapsed(i) {
            let widget = widget_creator(item, i);
            entry = entry.push(widget);
        }

        entry = entry.push(Rule::horizontal(4.));

        let collapse_text = if state.is_node_collapsed(i) { ">" } else { "v" };

        let collapse_button = widget::button(
            widget::text(collapse_text)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center))
            .on_press(Message::Input(callback_channel(ListEvent::Collapse(i, !state.is_node_collapsed(i)))))
            .style(theme::Button::Secondary)
            .height(Length::Fixed(button_size))
            .width(Length::Fixed(button_size))
            .padding(0);

        let collapse_button = widget::container(collapse_button)
            .padding(sidebar_padding);

        let entry = Row::new()
            .push(collapse_button)
            .push(entry);

        content = content.push(entry);
    }

    if data.len() > 0 {
        let add_button = widget::button(
            widget::text("+")
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center))
            .on_press(Message::Input(callback_channel(ListEvent::Add(AddLocation::Bottom))))
            .style(theme::Button::Positive)
            .height(Length::Fixed(button_size))
            .width(Length::Fixed(button_size))
            .padding(0);

        let add_row = Row::new()
            .push(widget::text("")
                .width(Length::Fixed(button_size + 2. * sidebar_padding as f32)))
            .push(add_button);

        content = content.push(add_row);
    }

    widget::container(
        Column::new()
            .push(header)
            .push(content)
            .spacing(SPACING_SMALL)
        ).into()
}

pub fn handle_list_event<Event, T, State, FEdit>(
    list_event: ListEvent<Event>,
    data: &mut Vec<T>,
    state: &mut State,
    mut edit_callback: FEdit)
where
    T: Default,
    State: ListState,
    FEdit: FnMut(&mut Vec<T>, Event, usize)
{
    use ListEvent::*;
    match list_event {
        Add(location) => {
            use AddLocation::*;
            match location {
                Top => {
                    data.insert(0, T::default());
                    state.add(0);
                },
                Bottom => {
                    data.push(T::default());
                    state.add(data.len() - 1);
                },
                Middle(index) => {
                    data.insert(index, T::default());
                    state.add(index);
                },
            }
        }
        Remove(index) => { data.remove(index); },
        Move(direction, index) => {
            use MoveDirection::*;
            match direction {
                Up => if index > 0 {
                    data.swap(index, index - 1)
                },
                Down => if index < data.len() - 1 {
                    data.swap(index, index + 1)
                }
            }
        },
        Collapse(index, is_collapsed) => {
            dbg!(is_collapsed);
            state.set_collapsed(index, is_collapsed);
        },
        Edit(item, index) => edit_callback(data, item, index)
    }
}

pub trait ListState {
    fn is_node_collapsed(&self, index: usize) -> bool;
    fn set_collapsed(&mut self, index: usize, collapsed: bool);
    fn add(&mut self, index: usize);
    fn remove(&mut self, index: usize);
}