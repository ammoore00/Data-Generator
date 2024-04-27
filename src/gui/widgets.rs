use iced::{Alignment, Application, Element, Length};
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
    Add(i32),
    Remove(i32),
    Drag,
    Drop,
    Edit(T)
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
        .push(widget::text(label))
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
        .push(widget::text(label))
        .push(button_true)
        .push(button_false)
        .align_items(Alignment::Center)
}

pub fn list<'a, T, FWidget, FCallback>(
    label: &str,
    state: &Vec<T>,
    widget_creator: FWidget,
    callback_channel: FCallback
) -> Element<'a, Message, <ApplicationWindow as Application>::Theme>
where FWidget: Fn(&T) -> Element<'a, Message, <ApplicationWindow as Application>::Theme>,
      FCallback: Fn(&Vec<T>) -> WidgetCallbackChannel + 'a,
{
    let name = widget::text(label);
    let add_top = widget::button("(+)")
        .on_press(Message::Input(callback_channel(state)))
        .style(theme::Button::Positive);
    let header = Row::new()
        .push(name)
        .push(add_top)
        .align_items(Alignment::Center)
        .spacing(10);

    let mut content = Column::new();

    for item in state {
        let widget = widget_creator(item);
        content = content.push(widget);
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