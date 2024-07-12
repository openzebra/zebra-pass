//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::widget::{component, pick_list, Button, Column, Component, Container, Row};
use iced::{Element, Renderer, Theme};
use rust_i18n::t;
use zebra_lib::settings::appearance;

pub struct GeneralSettings<'a, Message>
where
    Message: Clone,
{
    theme: appearance::Themes,
    on_copy: Option<Box<dyn Fn(String) -> Message + 'a>>,
}

#[derive(Debug, Clone)]
pub enum Event {
    ExportDatabase,
    ChangeTheme(appearance::Themes),
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
        Self {
            on_copy: None,
            theme: appearance::Themes::Auto,
        }
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
            Event::ChangeTheme(theme) => {
                self.theme = theme;

                None
            }
        }
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer> {
        let theme_pick_list = pick_list(
            [
                appearance::Themes::Auto,
                appearance::Themes::Light,
                appearance::Themes::Dark,
            ],
            Some(self.theme.clone()), // TODO: remove unwrap..
            Event::ChangeTheme,
        );
        let col = Row::new().push(theme_pick_list);

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
