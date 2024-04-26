use iced::{Alignment, Application, Element};
use iced::theme::{Button, PaneGrid};
use iced::widget::{self, pane_grid, Row};
use iced::widget::pane_grid::Axis;
use crate::gui::datapack::DatapackCallbackType;
use crate::gui::window::{ApplicationWindow, Message};

#[derive(Debug, Clone)]
pub enum WidgetCallbackChannel {
    PackInfo(DatapackCallbackType)
}

//------------//

#[derive(Debug, Clone)]
pub enum ListEvent<T> {
    Add(i32),
    Remove(i32),
    Drag,
    Drop,
    Edit(T)
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

pub fn list<'a, T, FWidget, FCallback>(
    label: &str,
    state: Vec<T>,
    widget_creator: FWidget,
    callback_channel: FCallback
) -> Element<'a, Message, <ApplicationWindow as Application>::Theme>
where FWidget: Fn(T) -> Element<'a, Message, <ApplicationWindow as Application>::Theme>,
      FCallback: Fn(Vec<T>) -> WidgetCallbackChannel + 'a,
{
    todo!()
    //let pane_grid = PaneGrid::new();
}

pub fn list_pane_state(count: usize) -> Option<pane_grid::Configuration<ListPaneState>> {
    match count {
        0 => None,
        1 => {
            let pane = ListPaneState::new(0);
            Some(pane_grid::Configuration::Pane(pane))
        }
        2 => {
            let first = ListPaneState::new(0);
            let second = ListPaneState::new(1);

            Some(pane_grid::Configuration::Split {
                axis: Axis::Horizontal,
                ratio: 0.5,
                a: Box::new(pane_grid::Configuration::Pane(first)),
                b: Box::new(pane_grid::Configuration::Pane(second)),
            })
        }
        count => {
            let next_pane = ListPaneState::new(count - 1);
            let prev_state = list_pane_state(count - 1).expect("Invalid state for list pane creation");

            Some(pane_grid::Configuration::Split {
                axis: Axis::Horizontal,
                ratio: 1. / (count as f32),
                a: Box::new(prev_state),
                b: Box::new(pane_grid::Configuration::Pane(next_pane))
            })
        }
    }
}

//------------//

#[derive(Debug, Clone)]
pub struct ListPaneState {
    index: usize,
    //data: T
}

impl ListPaneState {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            //data
        }
    }
}