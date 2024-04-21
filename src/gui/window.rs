use iced::{Application, Command, Element, executor, Length, Renderer, Sandbox, Theme};
use iced::alignment::Vertical;
use iced::theme::Button;
use iced::widget::{container, text, button, column as col, Rule, row, Container};
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

        let txt = text(format!("{:#?}", datapack));
        let btn = button(text("Switch pack"))
            .on_press(Message::SwitchPacks)
            .style(Button::Primary);

        let header_menu: Container<'_, Message, Theme, Renderer> = container(row![
                ])
            .width(Length::Fill)
            .height(Length::FillPortion(5))
            .center_x()
            .align_y(Vertical::Top);

        let file_browser: Container<'_, Message, Theme, Renderer> = container(col![
            text(title),
            btn,
        ].align_items(iced::Alignment::Center).spacing(10));
        let content: Container<'_, Message, Theme, Renderer> = container(col![
            txt
        ].align_items(iced::Alignment::Center).spacing(10));
        let preview: Container<'_, Message, Theme, Renderer> = container(col![

        ]);

        let main_view: Container<'_, Message, Theme, Renderer> = container(row![
            file_browser,
            content,
            preview
        ]);

        let total_window = col![
            header_menu,
            main_view
        ];

        container(total_window)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(40)
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