//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::theme::Theme;
use iced::widget::{button, column, text, Column};
use iced::{Alignment, Color, Element, Length, Sandbox, Settings};

#[derive(Default)]
pub struct App {
    theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Messages {}

impl Sandbox for App {
    type Message = Messages;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        "Zebra Password manager".into()
    }

    fn update(&mut self, message: Self::Message) {}

    fn view(&self) -> Element<'_, Self::Message> {
        column![text("test").size(50),].into()
    }
}
