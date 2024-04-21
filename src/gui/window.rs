use iced::{Application, Command, Element, executor, Length, Sandbox, Theme};
use iced::alignment::Vertical;
use iced::widget::{container, text, button, column as col, Rule};
use crate::data::datapack::{Datapack, SerializableDatapack};

pub struct ApplicationWindow {
    default: Datapack,
    terralith: Datapack,
    is_default: bool
}

impl Default for ApplicationWindow {
    fn default() -> Self {
        let filepath_default = "data/1-20-4.zip";

        let ser_default = SerializableDatapack::from_zip(filepath_default).unwrap();
        let default = Datapack::try_from(ser_default).unwrap();

        let filepath_terralith = "data/Terralith_1.20_v2.4.11.zip";

        let ser_terralith = SerializableDatapack::from_zip(filepath_terralith).unwrap();
        let terralith = Datapack::try_from(ser_terralith).unwrap();

        Self {
            default,
            terralith,
            is_default: true
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    SwitchPacks
}

impl Application for ApplicationWindow {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Gaia - Minecraft Datapack Generator")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SwitchPacks => {
                self.is_default = !self.is_default;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let datapack = if self.is_default { &self.default } else { &self.terralith };
        let title = if self.is_default { "Default" } else { "Terralith" };

        let txt = text(format!("{:#?}", datapack));
        let btn = button(text("Switch pack")).on_press(Message::SwitchPacks);

        let col = col![
            btn,
            text(title),
            Rule::horizontal(4.),
            txt,
        ].align_items(iced::Alignment::Center).spacing(10);

        container(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Vertical::Top)
            .center_x()
            .padding(40)
            .into()
    }
}