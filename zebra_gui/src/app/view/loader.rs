//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use iced::{
    widget::{button, text},
    Alignment, Command, Length, Subscription,
};
use zebra_ui::widget::*;

pub struct Loader {
    error: Option<String>,
}

#[derive(Clone, Copy, Debug)]
pub enum LoadMessage {
    Test,
}

impl Loader {
    pub fn new() -> Self {
        Self { error: None }
    }

    pub fn stop(&self) {}

    pub fn subscription(&self) -> Subscription<LoadMessage> {
        // Subscription::none()
        Subscription::batch([])
    }

    pub fn update(&mut self, message: LoadMessage) -> Command<LoadMessage> {
        dbg!("updated");
        match message {
            _ => Command::none(),
        }
    }

    pub fn view(&self) -> Element<LoadMessage> {
        let message = match &self.error {
            Some(err) => text(err).size(25),
            None => text("Loading...").size(25),
        }
        .horizontal_alignment(iced::alignment::Horizontal::Center);

        let row = Row::new()
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .push(button(text("test").size(14)).on_press(LoadMessage::Test))
            .push(message);

        Column::new()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(row)
            .into()
    }
}
