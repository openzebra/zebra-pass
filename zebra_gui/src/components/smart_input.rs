//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use std::sync::{Arc, Mutex};

use iced::widget::{component, text_input, Component};
use zebra_ui::style::Theme;
use zebra_ui::widget::*;

#[derive(Debug)]
pub struct SmartInputState {
    pub secured: bool,
    pub placeholder: String,
    pub value: String,
    pub label: String,
}

impl<'a> Default for SmartInputState {
    fn default() -> Self {
        Self {
            secured: false,
            placeholder: String::new(),
            value: String::new(),
            label: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct SmartInput<Message>
where
    Message: Clone,
{
    state: Arc<Mutex<SmartInputState>>,
    showed_secure_flag: bool,
    on_reload: Option<Message>,
    on_copy: Option<Message>,
}

#[derive(Debug, Clone)]
pub enum Event {
    Copy,
    Reload,
    ShowHideSecure,
    HandleInput(String),
}

impl<Message: Clone> SmartInput<Message>
where
    Message: Clone,
{
    pub fn new(state: Arc<Mutex<SmartInputState>>) -> Self {
        Self {
            state,
            showed_secure_flag: false,
            on_reload: None,
            on_copy: None,
        }
    }

    pub fn set_reload(mut self, reload_msg: Message) -> Self {
        self.on_reload = Some(reload_msg);

        self
    }

    pub fn set_copy(mut self, copy_msg: Message) -> Self {
        self.on_copy = Some(copy_msg);

        self
    }
}

impl<Message> Component<Message, Theme, Renderer> for SmartInput<Message>
where
    Message: Clone,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::Copy => self.on_copy.clone(),
            Event::Reload => self.on_reload.clone(),
            Event::ShowHideSecure => {
                self.showed_secure_flag = !self.showed_secure_flag;

                None
            }
            Event::HandleInput(v) => {
                match self.state.lock() {
                    Ok(mut state) => {
                        state.value = v;
                    }
                    Err(e) => {
                        // TODO: errors hanlder..
                        dbg!(e);
                    }
                }

                None
            }
        }
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer> {
        // TODO: remove unwrap..
        let state = self.state.lock().unwrap();
        // TODO: Remove clone.
        let label = Text::new(state.label.clone()).size(12);
        let input = text_input(&state.placeholder, &state.value)
            .size(14)
            .padding(4)
            .secure(state.secured && !self.showed_secure_flag)
            .on_input(Event::HandleInput)
            .style(zebra_ui::style::text_input::TextInput::Transparent);
        let col = Column::new().push(label).push(input);
        let mut row = Row::new().align_items(iced::Alignment::Center).push(col);

        if state.secured {
            let icon = if self.showed_secure_flag {
                zebra_ui::image::open_eye_icon()
            } else {
                zebra_ui::image::close_eye_icon()
            }
            .height(25)
            .width(25);
            let eye_btn = Button::new(icon)
                .padding(0)
                .on_press(Event::ShowHideSecure)
                .style(zebra_ui::style::button::Button::Transparent);
            row = row.push(eye_btn);
        }

        if self.on_copy.is_some() {
            let copy_btn = Button::new(zebra_ui::image::copy_icon().height(25).width(25))
                .padding(0)
                .on_press(Event::Copy)
                .style(zebra_ui::style::button::Button::Transparent);
            row = row.push(copy_btn);
        }

        if self.on_reload.is_some() {
            let reload_btn = Button::new(zebra_ui::image::reload_icon().height(30).width(30))
                .padding(0)
                .on_press(Event::Reload)
                .style(zebra_ui::style::button::Button::Transparent);
            row = row.push(reload_btn);
        }

        Container::new(row)
            .padding(3)
            .style(zebra_ui::style::container::Container::SecondaryRoundedBox)
            .into()
    }
}

impl<'a, Message> From<SmartInput<Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(form: SmartInput<Message>) -> Self {
        component(form)
    }
}
