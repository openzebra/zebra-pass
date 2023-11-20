//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use iced::{widget::text, Alignment, Command, Length, Subscription};
use zebra_ui::widget::*;

use crate::GlobalMessage;

pub struct Loader {
    error: Option<String>,
}

impl Loader {
    pub fn new() -> Self {
        Self { error: None }
    }

    pub fn stop(&self) {}

    pub fn subscription(&self) -> Subscription<GlobalMessage> {
        Subscription::none()
    }

    pub fn update(&mut self, message: GlobalMessage) -> Command<GlobalMessage> {
        dbg!("updated");
        match message {
            _ => Command::none(),
        }
    }

    pub fn view(&self) -> Element<GlobalMessage> {
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
