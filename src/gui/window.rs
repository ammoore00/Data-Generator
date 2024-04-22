use std::fmt::Debug;
use iced::{Application, Command, Element, executor, Length, Renderer, Sandbox, Theme};
use iced::alignment::Vertical;
use iced::theme::Button;
use iced::widget::{container, text, Container, Row, Column, PaneGrid, pane_grid, button};
use iced::widget::pane_grid::{Axis, TitleBar};
use crate::data::datapack::{Datapack, SerializableDatapack};
use crate::gui::datapack;
use crate::gui::widgets::WidgetCallbackChannel;
use crate::gui::window::MainContentState::PackInfo;

#[derive(Debug, Clone)]
pub enum Message {
    // Program functionality
    SwitchPacks,
    Input(WidgetCallbackChannel),
    // Pane grid functionality
    ResizedPane(pane_grid::ResizeEvent),
    ClickedPane(pane_grid::Pane),
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

    panes: pane_grid::State<PaneState>,
    focus: Option<pane_grid::Pane>,
}

impl Default for ApplicationWindow {
    fn default() -> Self {
        let filepath_default = "data/1-20-4.zip";

        let ser_default = SerializableDatapack::from_zip(filepath_default).unwrap();
        let default = Datapack::try_from(ser_default).unwrap();

        let filepath_terralith = "data/Terralith_1.20_v2.4.11.zip";

        let ser_terralith = SerializableDatapack::from_zip(filepath_terralith).unwrap();
        let terralith = Datapack::try_from(ser_terralith).unwrap();

        let file_tree_pane = PaneState::new(PaneType::FileTree);
        let main_content_pain = PaneState::new(PaneType::MainContent);
        let preview_pane = PaneState::new(PaneType::Preview);

        let panes = pane_grid::State::with_configuration(
            pane_grid::Configuration::Split{
                axis: Axis::Vertical,
                ratio: 0.2,
                a: Box::new(pane_grid::Configuration::Pane(file_tree_pane)),
                b: Box::new(pane_grid::Configuration::Split{
                    axis: Axis::Vertical,
                    ratio: 0.66,
                    a: Box::new(pane_grid::Configuration::Pane(main_content_pain)),
                    b: Box::new(pane_grid::Configuration::Pane(preview_pane)),
                }),
            });

        Self {
            default,
            terralith,
            state: PackInfo(true),

            panes,
            focus: None
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
            ///////////////////////////////////////
            //------ Program Functionality ------//
            ///////////////////////////////////////
            Message::SwitchPacks => {
                if let PackInfo(is_default) = &self.state {
                    self.state = PackInfo(!is_default);
                }
            }
            Message::Input(callback_channel) => {
                match callback_channel {
                    WidgetCallbackChannel::PackInfo(callback_type) => {
                        let datapack = self.get_active_datapack_mut();
                        datapack::handle_datapack_update(datapack, callback_type);
                    }
                }
            }
            /////////////////////////////////////////
            //------ Pane Grid Functionality ------//
            /////////////////////////////////////////
            Message::ResizedPane(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
            Message::ClickedPane(pane) => {
                self.focus = Some(pane);
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let header_menu = self.get_header();

        let main_view = PaneGrid::new(&self.panes, |pane, state, is_maximized| {
            let mut title: Row<'_, Message, Theme, Renderer> = Row::new();

            title = match state.pane_type {
                PaneType::FileTree => {
                    let title_text = &self.get_active_datapack().name;
                    title.push(text(title_text))
                }
                PaneType::MainContent => {
                    title.push(text("Pack Info"))
                }
                PaneType::Preview => {
                    title.push(text("Json Preview"))
                }
            };

            let title_bar = TitleBar::new(title)
                .padding(5);

            pane_grid::Content::new(
            match state.pane_type {
                    PaneType::FileTree => self.get_file_browser(),
                    PaneType::MainContent => self.get_content_view(),
                    PaneType::Preview => self.get_preview(),
                })
                .title_bar(title_bar)
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .on_click(Message::ClickedPane)
        .on_resize(10, Message::ResizedPane);

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
    fn get_active_datapack(&self) -> &Datapack {
        let mut datapack = &self.default;
        if let PackInfo(is_default) = &self.state {
            if !*is_default {
                datapack = &self.terralith;
            }
        }

        datapack
    }

    fn get_active_datapack_mut(&mut self) -> &mut Datapack {
        let mut datapack = &mut self.default;
        if let PackInfo(is_default) = &self.state {
            if !*is_default {
                datapack = &mut self.terralith;
            }
        }

        datapack
    }

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
        container(
            Column::new()
                .push(text("Pack Info"))
                .push(button(text("Switch pack"))
                    .on_press(Message::SwitchPacks)
                    .style(Button::Primary))
                .align_items(iced::Alignment::Start)
                .spacing(10)
                .width(Length::Fill)
                .height(Length::Fill))
            .width(Length::FillPortion(1))
            .height(Length::Fill)
            .padding(5)
    }

    fn get_content_view(&'a self) -> Container<'a, <ApplicationWindow as Application>::Message> {
        let datapack = self.get_active_datapack();

        container(
            Column::new()
                .push(datapack::get_datapack_gui(datapack))
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
        let datapack = self.get_active_datapack();

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
struct PaneState {
    pane_type: PaneType,
}

impl PaneState {
    fn new(pane_type: PaneType) -> Self {
        Self {
            pane_type,
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