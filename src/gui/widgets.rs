use iced::{Alignment, Application};
use iced::theme::Button;
use iced::widget::{self, Row};
use crate::gui::datapack::DatapackCallbackType;
use crate::gui::window::{ApplicationWindow, Message};

#[derive(Debug, Clone)]
pub enum WidgetCallbackChannel {
    PackInfo(DatapackCallbackType)
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
        .style(Button::Positive);
    let mut button_false = widget::button("False")
        .style(Button::Destructive);

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