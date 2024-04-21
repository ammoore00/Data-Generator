use iced::{Application, Command, Element, executor, Length, Renderer, Sandbox, Theme};
use iced::alignment::Vertical;
use iced::theme::Button;
use iced::widget::{container, text, button, Rule, Container, Row, Column};
use crate::data::datapack::{Datapack, SerializableDatapack};
use crate::gui::window::MainContentState::PackInfo;

pub struct ApplicationWindow {
    default: Datapack,
    terralith: Datapack,
    state: MainContentState
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
            state: MainContentState::PackInfo(true)
        }
    }
}

impl Application for ApplicationWindow {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), iced::window::maximize(iced::window::Id::MAIN, true))
    }

    fn title(&self) -> String {
        String::from("Gaia - Minecraft Datapack Generator")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SwitchPacks => {
                if let PackInfo(is_default) = &self.state {
                    self.state = PackInfo(!is_default);
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let (datapack, title) = if let PackInfo(is_default) = &self.state {
            if *is_default {
                (&self.default, "Default")
            }
            else {
                (&self.terralith, "Terralith")
            }
        }
        else {
            (&self.default, "Default")
        };

        //: Container<'_, Message, Theme, Renderer>

        let header_menu = container(
            Row::new()
                .push(text("File"))
                .push(text("Edit"))
                .align_items(iced::Alignment::Start).spacing(10))
            .width(Length::Fill)
            .height(Length::Fixed(25.))
            .center_x()
            .align_y(Vertical::Top)
            .style(iced::theme::Container::Box);

        let file_browser = container(
            Column::new()
                .push(text(title))
                .align_items(iced::Alignment::Center).spacing(10))
            .width(Length::FillPortion(1))
            .height(Length::Fill);

        let content = container(
            Column::new()
                .push(button(text("Switch pack"))
                    .on_press(Message::SwitchPacks)
                    .style(Button::Primary))
                .align_items(iced::Alignment::Center).spacing(10))
            .style(iced::theme::Container::Box)
            .width(Length::FillPortion(4))
            .height(Length::Fill);

        let preview = container(
            Column::new()
                .push(text(format!("{:#?}", datapack)))
                .align_items(iced::Alignment::Center).spacing(10))
            .width(Length::FillPortion(1))
            .height(Length::Fill);

        let main_view = container(
            Row::new()
                .push(file_browser)
                .push(content)
                .push(preview)
                .align_items(iced::Alignment::Center).spacing(10))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y();

        let total_window = Column::new()
            .push(header_menu)
            .push(main_view);

        container(total_window)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

//------------//

#[derive(Debug, Clone, Copy)]
pub enum Message {
    SwitchPacks
}

//------------//

#[derive(Debug, Clone, Copy)]
pub enum MainContentState {
    PackInfo(bool),
    Biome
}