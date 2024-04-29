use iced::{Element, Font, widget};
use iced::widget::Text;
use crate::gui::window::Message;

pub const TRASH_ICON: char = '\u{E800}';

pub fn icon<'a>(code_point: char) -> Text<'a> {
    widget::text(code_point).font(Font::with_name("icons"))
}