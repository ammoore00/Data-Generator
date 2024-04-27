use iced::{Alignment, Application, Element, Length, settings};
use iced::alignment::{Horizontal, Vertical};
use iced::theme;
use iced::widget::{self, Column, Row, Rule};
use crate::gui::pack_info::DatapackCallbackType;
use crate::gui::window::{ApplicationWindow, Message};

pub(crate) static SPACING_SMALL: u16 = 3;
pub(crate) static SPACING_LARGE: u16 = 6;

pub(crate) static STANDARD_RULE_WIDTH: f32 = 4.;

//------------//

#[derive(Debug, Clone)]
pub enum WidgetCallbackChannel {
    PackInfo(DatapackCallbackType)
}

//------------//

pub fn text_editor<'a, F>(
    label: &str,
    default: &str,
    state: &str,
    callback_channel: F
) -> Row<'a, Message, <ApplicationWindow as Application>::Theme>
where F: Fn(String) -> WidgetCallbackChannel + 'a {
    println!("{:?}", &state);
    let state = state.replace("\n", "\\n");
    println!("{:?}", &state);

    Row::new()
        .push(widget::text(format!("{label}:")))
        .push(widget::text_input(default, &*state)
            .on_input(move |text| {
                Message::Input(callback_channel(text.replace("\\n", "\n")))
            }))
        .align_items(Alignment::Center)
        .spacing(SPACING_LARGE)
}

//------------//

pub fn boolean_toggle<'a, F>(
    label: &str,
    state: bool,
    callback_channel: F
) -> Row<'a, Message, <ApplicationWindow as Application>::Theme>
where F: Fn(bool) -> WidgetCallbackChannel + 'a {
    let mut button_true = widget::button("True");
    let mut button_false = widget::button("False");

    if state {
        button_true = button_true
            .on_press(Message::Input(callback_channel(true)))
            .style(theme::Button::Positive);
        button_false = button_false
            .on_press(Message::Input(callback_channel(false)))
            .style(theme::Button::Secondary);
    }
    else {
        button_true = button_true
            .on_press(Message::Input(callback_channel(true)))
            .style(theme::Button::Secondary);
        button_false = button_false
            .on_press(Message::Input(callback_channel(false)))
            .style(theme::Button::Destructive);
    }

    // TODO: Style

    Row::new()
        .push(widget::text(format!("{label}: ")))
        .push(button_true)
        .push(button_false)
        .align_items(Alignment::Center)
}

pub fn boolean_toggle_optional<'a, F>(
    label: &str,
    state: Option<bool>,
    callback_channel: F
) -> Row<'a, Message, <ApplicationWindow as Application>::Theme>
where F: Fn(Option<bool>) -> WidgetCallbackChannel + 'a {
    let mut button_true = widget::button("True");
    let mut button_false = widget::button("False");

    if let Some(state) = state {
        if state {
            button_true = button_true
                .on_press(Message::Input(callback_channel(None)))
                .style(theme::Button::Positive);
            button_false = button_false
                .on_press(Message::Input(callback_channel(Some(false))))
                .style(theme::Button::Secondary);
        }
        else {
            button_true = button_true
                .on_press(Message::Input(callback_channel(Some(true))))
                .style(theme::Button::Secondary);
            button_false = button_false
                .on_press(Message::Input(callback_channel(None)))
                .style(theme::Button::Destructive);
        }
    }
    else {
        button_true = button_true
            .on_press(Message::Input(callback_channel(Some(true))))
            .style(theme::Button::Secondary);
        button_false = button_false
            .on_press(Message::Input(callback_channel(Some(false))))
            .style(theme::Button::Secondary);

    }

    // TODO: Style

    Row::new()
        .push(widget::text(format!("{label}: ")))
        .push(button_true)
        .push(button_false)
        .align_items(Alignment::Center)
}

//------------//

pub fn list<'a, T, EditEventType, InlineWidgetCreator, MessageCallback, State>(
    label: &str,
    data: &Vec<T>,
    list_state: &ListState,
    content_state: &State,
    settings: ListSettings<'a, T>,
    inline_widget_creator: InlineWidgetCreator,
    callback_channel: MessageCallback
) -> Element<'a, Message, <ApplicationWindow as Application>::Theme>
where
    T: Default,
    InlineWidgetCreator: Fn(&T, usize, bool, &State) -> Option<Element<'a, Message, <ApplicationWindow as Application>::Theme>>,
    MessageCallback: Fn(ListEvent<EditEventType>) -> WidgetCallbackChannel + 'a,
{
    let button_size = 24.;
    let sidebar_padding = 4;

    let name = widget::text(label);
    let add_top = widget::button(" + ")
        .on_press(Message::Input(callback_channel(ListEvent::Add(0))))
        .style(theme::Button::Positive);
    let mut header = Row::new()
        .push(name).push(add_top)
        .align_items(Alignment::Center)
        .spacing(SPACING_LARGE);

    let mut content = Column::new()
        .spacing(SPACING_SMALL);

    for i in 0..data.len() {
        let item = &data[i];
        let collapsed = list_state.is_node_collapsed(i);

        let add_button = widget::button(
                widget::text("+")
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center))
            .on_press(Message::Input(callback_channel(ListEvent::Add(i + 1))))
            .style(theme::Button::Positive)
            .height(Length::Fixed(button_size))
            .width(Length::Fixed(button_size))
            .padding(0);
        let remove_button = widget::button(
                widget::text("-")
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center))
            .on_press(Message::Input(callback_channel(ListEvent::Remove(i))))
            .style(theme::Button::Destructive)
            .height(Length::Fixed(button_size))
            .width(Length::Fixed(button_size))
            .padding(0);

        let mut add_remove_buttons = Row::new()
            .push(add_button);

        if !settings.required || data.len() > 1 {
            add_remove_buttons = add_remove_buttons.push(remove_button);
        }

        let mut up_button = widget::button(
                widget::text("^")
                    .size(11)
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center))
            .style(theme::Button::Secondary)
            .height(Length::Fixed(button_size * 2. / 3.))
            .width(Length::Fixed(button_size))
            .padding(0);
        let mut down_button = widget::button(
                widget::text("v")
                    .size(9)
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center))
            .style(theme::Button::Secondary)
            .height(Length::Fixed(button_size * 2. / 3.))
            .width(Length::Fixed(button_size))
            .padding(0);

        if i > 0 {
            up_button = up_button.on_press(Message::Input(callback_channel(ListEvent::Move(MoveDirection::Up, i))))
        }
        if i < data.len() - 1 {
            down_button = down_button.on_press(Message::Input(callback_channel(ListEvent::Move(MoveDirection::Down, i))))
        }

        let move_buttons = Column::new()
            .push(up_button)
            .push(down_button);

        let mut controls = Row::new()
            .push(add_remove_buttons)
            .push(move_buttons)
            .align_items(Alignment::Center)
            .spacing(SPACING_SMALL);

        let inline_widget = inline_widget_creator(item, i, collapsed, content_state);

        let entry = if let ListInlineState::Extended(extended_widget) = &settings.inline_state {
            if let Some(inline_widget) = inline_widget {
                controls = controls.push(inline_widget);
            }

            let mut entry = Column::new().push(controls);
            if !collapsed {
                entry = entry.push(extended_widget(item, i));
            }
            entry
        }
        else {
            if let Some(inline_widget) = inline_widget {
                controls = controls.push(inline_widget);
            }
            Column::new()
                .push(controls)
        }
        .push(Rule::horizontal(STANDARD_RULE_WIDTH))
        .spacing(SPACING_LARGE);

        let entry = if let ListInlineState::Inline = settings.inline_state {
            Row::new()
                .push(widget::text("")
                    .width(Length::Fixed(button_size + 2. * sidebar_padding as f32)))
                .push(entry)
        }
        else {
            let collapse_text = if collapsed { ">" } else { "v" };

            let collapse_button = widget::button(
                widget::text(collapse_text)
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center))
                .on_press(Message::Input(callback_channel(ListEvent::Collapse(i, !list_state.is_node_collapsed(i)))))
                .style(theme::Button::Secondary)
                .height(Length::Fixed(button_size))
                .width(Length::Fixed(button_size))
                .padding(0);

            let collapse_button = widget::container(collapse_button)
                .padding(sidebar_padding);

            Row::new()
                .push(collapse_button)
                .push(entry)
        };

        content = content.push(entry);
    }

    widget::container(
        Column::new()
            .push(header)
            .push(Rule::horizontal(STANDARD_RULE_WIDTH))
            .push(content)
            .spacing(SPACING_SMALL)
        ).into()
}

pub fn handle_list_event<Event, T, FEdit>(
    list_event: ListEvent<Event>,
    data: &mut Vec<T>,
    state: &mut ListState,
    mut edit_callback: FEdit)
where
    T: Default,
    FEdit: FnMut(&mut Vec<T>, Event, usize)
{
    use ListEvent::*;
    match list_event {
        Add(index) => {
            data.insert(index, T::default());
            state.add(index);
        }
        Remove(index) => {
            data.remove(index);
            state.remove(index);
        },
        Move(direction, index) => {
            use MoveDirection::*;
            match direction {
                Up => if index > 0 {
                    data.swap(index, index - 1)
                },
                Down => if index < data.len() - 1 {
                    data.swap(index, index + 1)
                }
            }
        },
        Collapse(index, is_collapsed) => state.set_collapsed(index, is_collapsed),
        Edit(item, index) => edit_callback(data, item, index)
    }
}

//------------//

#[derive(Debug, Clone)]
pub struct ListState {
    collapsed: Vec<bool>
}

impl ListState {
    pub fn new(count: usize) -> Self {
        Self {
            collapsed: vec![false; count]
        }
    }

    fn is_node_collapsed(&self, index: usize) -> bool {
        *self.collapsed.get(index).unwrap_or(&false)
    }

    fn set_collapsed(&mut self, index: usize, collapsed: bool) {
        if let Some(mut val) = self.collapsed.get_mut(index) {
            *val = collapsed;
        }
    }

    fn add(&mut self, index: usize) {
        self.collapsed.insert(index, false);
    }

    fn remove(&mut self, index: usize) {
        self.collapsed.remove(index);
    }
}

//------------//

pub struct ListSettings<'a, T>
where T: Default {
    pub required: bool,
    pub inline_state: ListInlineState<'a, T>
}

impl<'a, T> Default for ListSettings<'a, T>
where T: Default {
    fn default() -> Self {
        Self {
            required: false,
            inline_state: ListInlineState::Inline
        }
    }
}

//------------//

pub enum ListInlineState<'a, T>
where T: Default {
    Inline,
    Extended(Box<dyn Fn(&T, usize) -> Element<'a, Message, <ApplicationWindow as Application>::Theme>>)
}

//------------//

#[derive(Debug, Clone)]
pub enum ListEvent<T> {
    Add(usize),
    Remove(usize),
    Move(MoveDirection, usize),
    Collapse(usize, bool),
    Edit(T, usize)
}

//------------//

#[derive(Debug, Clone)]
enum MoveDirection {
    Up,
    Down
}