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

pub fn list<'a, DataType, EditEventType, WidgetCreator, MessageCallback>(
    label: &str,
    state: &Vec<DataType>,
    widget_creator: WidgetCreator,
    callback_channel: MessageCallback
) -> Element<'a, Message, <ApplicationWindow as Application>::Theme>
where
    DataType: Default,
    WidgetCreator: Fn(&DataType, usize) -> Element<'a, Message, <ApplicationWindow as Application>::Theme>,
    MessageCallback: Fn(ListEvent<EditEventType>) -> WidgetCallbackChannel + 'a,
{
    let name = widget::text(label);
    let add_top = widget::button(" + ")
        .on_press(Message::Input(callback_channel(ListEvent::Add(AddLocation::Top))))
        .style(theme::Button::Positive);
    let mut header = Row::new()
        .push(name)
        .align_items(Alignment::Center)
        .spacing(SPACING_LARGE);

    if state.len() == 0 {
        header = header.push(add_top)
    }

    let mut content = Column::new()
        .spacing(SPACING_SMALL);

    for i in 0..state.len() {
        let item = &state[i];

        let widget = widget_creator(item, i);

        let button_size = 24.;

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
        if i < state.len() - 1 {
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
            .spacing(SPACING_SMALL);

        entry = entry
            .push(controls)
            .push(widget)
            .push(Rule::horizontal(4.));

        let collapse_button = widget::button(
            widget::text(">")
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center))
            .style(theme::Button::Secondary)
            .height(Length::Fixed(button_size))
            .width(Length::Fixed(button_size))
            .padding(0);

        let sidebar = Column::new()
            .push(collapse_button)
            .push(Rule::vertical(STANDARD_RULE_WIDTH))
            .align_items(Alignment::Center);

        let entry = Row::new()
            .push(sidebar)
            .push(entry);

        content = content.push(entry);
    }

    if state.len() > 0 {
        content = content.push(widget::button(" + ")
            .on_press(Message::Input(callback_channel(ListEvent::Add(AddLocation::Bottom))))
            .style(theme::Button::Positive));
    }

    /*
    let content = Row::new()
        .push(widget::container(Rule::vertical(STANDARD_RULE_WIDTH))
            .center_x()
            .width(Length::Fixed(25.)))
            .height(Length::Fill)
        .push(content);
     */

    widget::container(
        Column::new()
            .push(header)
            .push(content)
            .spacing(SPACING_SMALL)
        )
        .into()
}

pub fn handle_list_event<E, T, F>(
    list_event: ListEvent<E>,
    data: &mut Vec<T>,
    mut edit_callback: F)
where
    T: Default,
    F: FnMut(&mut Vec<T>, E, usize)
{
    use ListEvent::*;
    match list_event {
        Add(location) => {
            use AddLocation::*;
            match location {
                Top => data.insert(0, T::default()),
                Bottom => data.push(T::default()),
                Middle(index) => data.insert(index, T::default()),
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
        Edit(item, index) => edit_callback(data, item, index)
    }
}