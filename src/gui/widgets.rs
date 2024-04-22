use iced::{Alignment, Application};
use iced::theme::Button;
use iced::widget::{button, Row, text, text_input};
use crate::gui::window::{ApplicationWindow, Message};

pub fn text_editor<'a>(label: &str, default: &str, reference: &str) -> Row<'a, Message, <ApplicationWindow as Application>::Theme> {
    Row::new()
        .push(text(label))
        .push(text_input(default, reference)
            .on_input(|text| {
                Message::Input(text)
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