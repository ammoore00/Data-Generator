use iced::{Application, Command, Element, executor, Length, Renderer, Sandbox, Theme};
use iced::alignment::Vertical;
use iced::theme::Button;
use iced::widget::{container, text, button, Rule, Container, Row, Column, PaneGrid, pane_grid};
use iced::widget::pane_grid::Axis;
use crate::data::datapack::{Datapack, SerializableDatapack};
use crate::gui::window::MainContentState::PackInfo;

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

//------------//

pub struct ApplicationWindow {
    default: Datapack,
    terralith: Datapack,
    state: MainContentState,

    panes: pane_grid::State<Pane>
}

impl Default for ApplicationWindow {
    fn default() -> Self {
        let filepath_default = "data/1-20-4.zip";

        let ser_default = SerializableDatapack::from_zip(filepath_default).unwrap();
        let default = Datapack::try_from(ser_default).unwrap();

        let filepath_terralith = "data/Terralith_1.20_v2.4.11.zip";

        let ser_terralith = SerializableDatapack::from_zip(filepath_terralith).unwrap();
        let terralith = Datapack::try_from(ser_terralith).unwrap();

        let file_tree_pane = Pane::new(PaneType::FileTree);
        let main_content_pain = Pane::new(PaneType::MainContent);
        let preview_pane = Pane::new(PaneType::Preview);

        let panes = pane_grid::State::with_configuration(
            pane_grid::Configuration::Split{
                axis: Axis::Vertical,
                ratio: 0.2,
                a: Box::new(pane_grid::Configuration::Pane(file_tree_pane)),
                b: Box::new(pane_grid::Configuration::Split{
                    axis: Axis::Vertical,
                    ratio: 0.75,
                    a: Box::new(pane_grid::Configuration::Pane(main_content_pain)),
                    b: Box::new(pane_grid::Configuration::Pane(preview_pane)),
                }),
            });

        Self {
            default,
            terralith,
            state: PackInfo(true),
            panes
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
        //: Container<'_, Message, Theme, Renderer>

        let header_menu = self.get_header();

        let main_view = PaneGrid::new(&self.panes, |id, pane, is_maximized| {
            pane_grid::Content::new(
                match pane.pane_type {
                    PaneType::FileTree => self.get_file_browser(),
                    PaneType::MainContent => self.get_content_view(),
                    PaneType::Preview => self.get_preview(),
                }
            )
        });

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

impl<'a> ApplicationWindow {
    fn get_header(&self) -> Container<'a, <ApplicationWindow as Application>::Message> {
        container(
            Row::new()
                .push(text("File"))
                .push(text("Edit"))
                .align_items(iced::Alignment::Start)
                .spacing(10)
                .width(Length::Fill)
                .height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fixed(25.))
            .center_x()
            .align_y(Vertical::Top)
            .style(iced::theme::Container::Box)
    }

    fn get_file_browser(&self) -> Container<'a, <ApplicationWindow as Application>::Message> {
        let title = if let PackInfo(is_default) = &self.state {
            if *is_default {
                "Default"
            }
            else {
                "Terralith"
            }
        }
        else {
            "Default"
        };

        container(
            Column::new()
                .push(text(title))
                .push(Rule::horizontal(4.))
                .push(text("Pack Info"))
                .align_items(iced::Alignment::Start)
                .spacing(10)
                .width(Length::Fill)
                .height(Length::Fill))
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .padding(5)
    }

    fn get_content_view(&self) -> Container<'a, <ApplicationWindow as Application>::Message> {
        container(
            Column::new()
                .push(button(text("Switch pack"))
                    .on_press(Message::SwitchPacks)
                    .style(Button::Primary))
                .align_items(iced::Alignment::Start)
                .spacing(10)
                .width(Length::Fill)
                .height(Length::Fill))
            .style(iced::theme::Container::Box)
            .width(Length::FillPortion(4))
            .height(Length::Fill)
            .padding(5)
    }

    fn get_preview(&self) -> Container<'a, <ApplicationWindow as Application>::Message> {
        let datapack = if let PackInfo(is_default) = &self.state {
            if *is_default {
                &self.default
            }
            else {
                &self.terralith
            }
        }
        else {
            &self.default
        };

        container(
            Column::new()
                .push(text(format!("{:#?}", datapack)))
                .align_items(iced::Alignment::Start)
                .spacing(10)
                .width(Length::Fill)
                .height(Length::Fill))
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .padding(5)
    }
}

//------------//

#[derive(Debug, Clone, Copy)]
struct Pane {
    pane_type: PaneType,
    pub is_pinned: bool,
}

impl Pane {
    fn new(pane_type: PaneType) -> Self {
        Self {
            pane_type,
            is_pinned: false,
        }
    }
}

//------------//

#[derive(Debug, Clone, Copy)]
enum PaneType {
    FileTree,
    MainContent,
    Preview
}