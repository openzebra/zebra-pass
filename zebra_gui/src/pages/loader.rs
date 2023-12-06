//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::Arc;

use iced::{widget::text, Alignment, Command, Length, Subscription};
use zebra_lib::{core::core::Core, errors::ZebraErrors};
use zebra_ui::widget::*;

use crate::gui::GlobalMessage;

use super::Page;

#[derive(Debug)]
pub struct Loader {
    error: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum LoadMessage {
    Synced,
}

impl Page for Loader {
    type Message = LoadMessage;

    fn new(_core: Arc<Core>) -> Result<Self, ZebraErrors> {
        Ok(Self { error: None })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&self, _message: LoadMessage) -> Command<GlobalMessage> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
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
