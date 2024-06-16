//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use std::borrow::Cow;

use iced::widget::{component, Button, Column, Component, Container, Row, Space, Text};
use iced::{Element, Length, Padding, Renderer, Theme};

pub struct SmartFields<'a, Message>
where
    Message: Clone,
{
    on_copy: Option<Box<dyn Fn(String) -> Message + 'a>>,
    container_padding: Padding,
    label_size: u16,
    value_size: u16,
    label: Cow<'a, str>,
    value: Cow<'a, str>,
}

#[derive(Debug, Clone)]
pub enum Event {}

impl<'a, Message> Default for SmartFields<'a, Message>
where
    Message: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> SmartFields<'a, Message>
where
    Message: Clone,
{
    pub fn new() -> Self {
        Self {
            container_padding: Padding::ZERO,
            on_copy: None,
            label_size: 16,
            value_size: 14,
            label: Cow::default(),
            value: Cow::default(),
        }
    }

    pub fn set_label(mut self, label: Cow<'a, str>) -> Self {
        self.label = label;
        self
    }

    pub fn set_value(mut self, value: Cow<'a, str>) -> Self {
        self.value = value;
        self
    }

    pub fn set_label_size(mut self, size: u16) -> Self {
        self.label_size = size;
        self
    }

    pub fn set_value_size(mut self, size: u16) -> Self {
        self.value_size = size;
        self
    }

    pub fn set_padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.container_padding = padding.into();
        self
    }

    pub fn on_copy<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(String) -> Message,
    {
        self.on_copy = Some(Box::new(callback));

        self
    }
}

impl<'a, Message> Component<Message, Theme, Renderer> for SmartFields<'a, Message>
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
        let title = Text::new(self.label.as_ref()).size(self.label_size);
        let value = Text::new(self.value.as_ref())
            .size(self.value_size)
            .style(zebra_ui::styles::text::muted);
        let col = Column::new().push(title).push(value);
        let row = Row::new().push(col);

        Container::new(row)
            .padding(self.container_padding)
            .width(Length::Fill)
            .into()
    }
}

impl<'a, Message> From<SmartFields<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(form: SmartFields<'a, Message>) -> Self {
        component(form)
    }
}
