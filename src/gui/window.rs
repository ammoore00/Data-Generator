use std::fmt::Debug;
use iced::{Application, Command, Element, executor, Length, Renderer, Sandbox, Theme};
use iced::alignment::Vertical;
use iced::theme::Button;
use iced::widget::{self, Container, Row, Column, PaneGrid, Rule};
use iced::widget::pane_grid::{self, Axis, TitleBar};
use crate::data::datapack::{Datapack, SerializableDatapack};
use crate::gui::{pack_info, widgets};
use crate::gui::pack_info::PackInfoState;
use crate::gui::widgets::WidgetCallbackChannel;

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

#[derive(Debug, Clone)]
pub enum MainContentState {
    PackInfo(PackInfoState),
    Biome
}

//------------//

pub struct ApplicationWindow {
    datapack: Datapack,
    state: MainContentState,

    panes: pane_grid::State<PaneState>,
    focus: Option<pane_grid::Pane>,
}

impl Default for ApplicationWindow {
    fn default() -> Self {
        let filepath_default = "data/1-20-4.zip";

        let serialized_datapack = SerializableDatapack::from_zip(filepath_default).unwrap();
        let datapack = Datapack::try_from(serialized_datapack).unwrap();

        let filepath_terralith = "data/Terralith_1.20_v2.4.11.zip";

        let datapack = SerializableDatapack::from_zip(filepath_terralith).unwrap();
        let datapack = Datapack::try_from(datapack).unwrap();

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

        let state = MainContentState::PackInfo(PackInfoState::new(&datapack));

        Self {
            datapack,
            state,

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
        use crate::gui::window::MainContentState::*;
        use crate::gui::window::Message::*;
        match message {
            ///////////////////////////////////////
            //------ Program Functionality ------//
            ///////////////////////////////////////
            SwitchPacks => {
                let filepath_default = "data/1-20-4.zip";
                let filepath_terralith = "data/Terralith_1.20_v2.4.11.zip";

                if self.datapack.name() == "1-20-4" {
                    let datapack = SerializableDatapack::from_zip(filepath_terralith).unwrap();
                    self.datapack = Datapack::try_from(datapack).unwrap();
                }
                else {
                    let datapack = SerializableDatapack::from_zip(filepath_default).unwrap();
                    self.datapack = Datapack::try_from(datapack).unwrap();
                }

                self.state = PackInfo(PackInfoState::new(&self.datapack));
            }
            Input(callback_channel) => {
                match callback_channel {
                    WidgetCallbackChannel::PackInfo(callback_type) => {
                        if let PackInfo(pack_info_state) = self.state.clone() {
                            let datapack = &mut self.datapack;
                            self.state = PackInfo(pack_info::handle_datapack_update(datapack, callback_type, pack_info_state));
                        }
                        else {
                            panic!("Illegal state - pack info callback requested while not in pack info state!")
                        }
                    }
                }
            }
            /////////////////////////////////////////
            //------ Pane Grid Functionality ------//
            /////////////////////////////////////////
            ResizedPane(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
            ClickedPane(pane) => {
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
                    let title_text = &self.datapack.name();
                    title.push(widget::text(title_text))
                }
                PaneType::MainContent => {
                    title.push(widget::text("Pack Info"))
                }
                PaneType::Preview => {
                    title.push(widget::text("Json Preview"))
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

        widget::container(total_window)
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
        widget::container(
            Row::new()
                .push(widget::text("File"))
                .push(widget::text("Edit"))
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
        widget::container(
            Column::new()
                .push(widget::text("Pack Info"))
                .push(widget::button(widget::text("Switch pack"))
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
        let datapack = &self.datapack;
        let pack_info_state = if let MainContentState::PackInfo(pack_info_state) = &self.state {
            pack_info_state
        }
        else {panic!("Main content hardcoded for pack info")};

        widget::container(
            Column::new()
                .push(pack_info::pack_info_gui(datapack, pack_info_state))
                .align_items(iced::Alignment::Start)
                .spacing(10)
                .width(Length::Fill)
                .height(Length::Fill))
            //.style(iced::theme::Container::Box)
            .width(Length::FillPortion(4))
            .height(Length::Fill)
            .padding(5)
    }

    fn get_preview(&self) -> Container<'a, <ApplicationWindow as Application>::Message> {
        let datapack = &self.datapack;

        widget::container(
            Column::new()
                .push(widget::text(format!("{:#?}", self.state)))
                .push(Rule::horizontal(widgets::STANDARD_RULE_WIDTH))
                .push(widget::text(format!("{:#?}", datapack)))
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
enum PaneType {
    FileTree,
    MainContent,
    Preview
}