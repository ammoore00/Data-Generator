use iced::{Alignment, Application, Element};
use iced::theme;
use iced::widget::{self, Column, Row, Rule};
use crate::gui::datapack::DatapackCallbackType;
use crate::gui::window::{ApplicationWindow, Message};

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
    Bottom
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
        .spacing(10)
}

pub fn boolean_toggle<'a, F>(
    label: &str,
    state: bool,
    callback_channel: F
) -> Row<'a, Message, <ApplicationWindow as Application>::Theme>
where F: Fn(bool) -> WidgetCallbackChannel + 'a {
    let mut button_true = widget::button("True")
        .style(theme::Button::Positive);
    let mut button_false = widget::button("False")
        .style(theme::Button::Destructive);

    if state {
        button_true = button_true.on_press(Message::Input(callback_channel(false)));
    }
    else {
        button_false = button_false.on_press(Message::Input(callback_channel(true)));
    }

    // TODO: Style

    Row::new()
        .push(widget::text(format!("{label}:")))
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
    WidgetCreator: Fn(&DataType, usize) -> Element<'a, Message, <ApplicationWindow as Application>::Theme>,
    MessageCallback: Fn(ListEvent<EditEventType>) -> WidgetCallbackChannel + 'a,
{
    let name = widget::text(label);
    let add_top = widget::button(" + ")
        .on_press(Message::Input(callback_channel(ListEvent::Add(AddLocation::Top))))
        .style(theme::Button::Positive);
    let header = Row::new()
        .push(name)
        .push(add_top)
        .align_items(Alignment::Center)
        .spacing(10);

    let mut content = Column::new()
        .spacing(10);

    for i in 0..state.len() {
        let item = &state[i];

        let widget = widget_creator(item, i);

        let remove_button = widget::button(" - ")
            .on_press(Message::Input(callback_channel(ListEvent::Remove(i))))
            .style(theme::Button::Destructive);
        let up_button = widget::button("^")
            .on_press(Message::Input(callback_channel(ListEvent::Move(MoveDirection::Up, i))))
            .style(theme::Button::Secondary);
        let down_button = widget::button("v")
            .on_press(Message::Input(callback_channel(ListEvent::Move(MoveDirection::Down, i))))
            .style(theme::Button::Secondary);

        let mut controls = Row::new().push(remove_button);

        if i > 0 {
            controls = controls.push(up_button);
        }
        if i < state.len() - 1 {
            controls = controls.push(down_button);
        }

        let mut entry = Column::new()
            .spacing(5);

        if i > 0 {
            entry = entry.push(Rule::horizontal(4.));
        }

        entry = entry
            .push(controls)
            .push(widget);

        content = content.push(entry);
    }

    if state.len() > 0 {
        content = content.push(widget::button(" + ")
            .on_press(Message::Input(callback_channel(ListEvent::Add(AddLocation::Bottom))))
            .style(theme::Button::Positive));
    }

    let header = Column::new()
        .push(header)
        .push(Rule::horizontal(4.));

    widget::container(
        Column::new()
            .push(header)
            .push(content)
        ).into()
}

pub fn handle_list_event<E, T, F>(
    list_event: ListEvent<E>,
    data: &mut Vec<T>,
    mut edit_callback: F)
where
    T: Default,
    F: FnMut(&mut Vec<T>, E, usize)
{
    match list_event {
        ListEvent::Add(location) => {
            match location {
                AddLocation::Top => {
                    data.insert(0, T::default())
                }
                AddLocation::Bottom => {
                    data.push(T::default())
                }
            }
        }
        ListEvent::Remove(index) => {
            data.remove(index);
        },
        ListEvent::Move(direction, index) => {
            match direction {
                MoveDirection::Up => if index > 0 {
                    data.swap(index, index - 1)
                },
                MoveDirection::Down => if index < data.len() - 1 {
                    data.swap(index, index + 1)
                }
            }
        },
        ListEvent::Edit(item, index) => edit_callback(data, item, index)
    }
}