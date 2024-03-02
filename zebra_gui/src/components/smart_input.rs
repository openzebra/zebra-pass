//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use iced::widget::{component, text_input, Component};
use std::sync::{Arc, Mutex};
use zebra_lib::errors::ZebraErrors;
use zebra_ui::style::Theme;
use zebra_ui::widget::*;

#[derive(Debug)]
pub struct SmartInputState {}

impl<'a> Default for SmartInputState {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct SmartInput<Message>
where
    Message: Clone,
{
    state: Arc<Mutex<SmartInputState>>,
    on_copy: Option<Message>,
}

#[derive(Debug, Clone)]
pub enum Event {
    Copy,
    HandleInput(String),
}

impl<Message> SmartInput<Message>
where
    Message: Clone,
{
    pub fn new(state: Arc<Mutex<SmartInputState>>) -> Result<Self, ZebraErrors> {
        Ok(Self {
            state,
            on_copy: None,
        })
    }

    pub fn set_on_copy(mut self, on_copy: Message) -> Self {
        self.on_copy = Some(on_copy);

        self
    }

    pub fn view_input(&self) -> Container<'_, Event> {
        let label = Text::new("name").size(12);
        let input = text_input("", "")
            .size(14)
            .padding(4)
            .secure(true)
            .on_input(Event::HandleInput)
            .style(zebra_ui::style::text_input::TextInput::Transparent);
        let col = Column::new().push(label).push(input);
        let copy_btn = Button::new(zebra_ui::image::copy_icon().height(25).width(25))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent);
        let reload_btn = Button::new(zebra_ui::image::reload_icon().height(30).width(30))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent);
        let row = Row::new()
            .align_items(iced::Alignment::Center)
            .push(col)
            .push(copy_btn)
            .push(reload_btn);

        Container::new(row)
            .padding(3)
            .style(zebra_ui::style::container::Container::SecondaryRoundedBox)
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
            Event::HandleInput(v) => {
                dbg!(v);

                None
            }
        }
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer> {
        self.view_input().into()
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
