//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::widget::{component, Column, Component, Container};
use iced::{Element, Renderer, Theme};
use rust_i18n::t;

pub struct GeneralSettings<'a, Message>
where
    Message: Clone,
{
    on_copy: Option<Box<dyn Fn(String) -> Message + 'a>>,
}

#[derive(Debug, Clone)]
pub enum Event {
    ExportDatabase,
}

impl<'a, Message> Default for GeneralSettings<'a, Message>
where
    Message: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message: Clone> GeneralSettings<'a, Message>
where
    Message: Clone,
{
    pub fn new() -> Self {
        Self { on_copy: None }
    }

    pub fn on_copy<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(String) -> Message,
    {
        self.on_copy = Some(Box::new(callback));

        self
    }
}

impl<'a, Message> Component<Message, Theme, Renderer> for GeneralSettings<'a, Message>
where
    Message: Clone,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::ExportDatabase => None,
        }
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer> {
        let col = Column::new();

        Container::new(col).into()
    }
}

impl<'a, Message> From<GeneralSettings<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(form: GeneralSettings<'a, Message>) -> Self {
        component(form)
    }
}
