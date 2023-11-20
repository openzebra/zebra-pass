//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use iced::{widget::text, Alignment, Command, Length, Subscription};
use zebra_ui::widget::*;

pub struct Loader {
    error: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum LoadMessage {
    Synced(),
}

impl Loader {
    pub fn new() -> Self {
        Self { error: None }
    }

    pub fn stop(&self) {}

    pub fn subscription(&self) -> Subscription<LoadMessage> {
        Subscription::none()
    }

    pub fn update<M>(&mut self, _message: LoadMessage) -> Command<M> {
        Command::none()
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
            .push(message);

        Column::new()
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(row)
            .into()
    }
}
