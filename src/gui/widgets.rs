use iced::{Alignment, Application};
use iced::theme::Button;
use iced::widget::{button, Row, text, text_input};
use crate::gui::datapack::DatapackCallbackType;
use crate::gui::window::{ApplicationWindow, Message};

#[derive(Debug, Clone)]
pub enum WidgetCallbackChannel {
    PackInfo(DatapackCallbackType)
}

//------------//

pub fn text_editor<'a, F>(label: &str, default: &str, reference: &str, callback_channel: F) -> Row<'a, Message, <ApplicationWindow as Application>::Theme>
where F: Fn(String) -> WidgetCallbackChannel + 'a {
    Row::new()
        .push(text(label))
        .push(text_input(default, reference)
            .on_input(move |text| {
                Message::Input(callback_channel(text))
            }))
        .align_items(Alignment::Center)
        .spacing(10)
}

pub fn true_false<'a>(label: &str, default: bool, reference: &bool) -> Row<'a, Message, <ApplicationWindow as Application>::Theme> {
    Row::new()
        .push(text(label))
        .push(button("True")
            .style(Button::Positive))
        .push(button("False")
            .style(Button::Destructive))
        .align_items(Alignment::Center)
}