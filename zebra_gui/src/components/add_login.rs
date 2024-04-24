//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::widget::{component, text_input, Button, Column, Component, Container, Row, Space, Text};
use iced::{Element, Renderer, Theme};

pub struct AddLogin<'a, Message>
where
    Message: Clone,
{
    read_only: bool,
    on_submit: Option<Box<dyn Fn(String) -> Message + 'a>>,
}

#[derive(Debug, Clone)]
pub enum Event {}

impl<'a, Message: Clone> AddLogin<'a, Message>
where
    Message: Clone,
{
    pub fn new() -> Self {
        Self {
            read_only: false,
            on_submit: None,
        }
    }
}

impl<'a, Message> Component<Message, Theme, Renderer> for AddLogin<'a, Message>
where
    Message: Clone,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {}
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer> {
        let row = Row::new();
        Container::new(row).into()
    }
}

impl<'a, Message> From<AddLogin<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(form: AddLogin<'a, Message>) -> Self {
        component(form)
    }
}
