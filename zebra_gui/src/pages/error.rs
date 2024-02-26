//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use crate::rust_i18n::t;
use iced::{widget::Column, Command, Length, Subscription};
use zebra_lib::{core::core::Core, errors::ZebraErrors};
use zebra_ui::widget::*;

use crate::gui::GlobalMessage;

use super::Page;

#[derive(Debug)]
pub struct ErrorPage {
    message: String,
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorPageMessage {}

impl Page for ErrorPage {
    type Message = ErrorPageMessage;

    fn new(_core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        Ok(Self {
            message: String::new(),
        })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<GlobalMessage> {
        // match message {};
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let title = Text::new(t!("error_page_title"))
            .size(18)
            .style(zebra_ui::style::text::Text::Info);
        let message = Text::new(&self.message)
            .size(14)
            .style(zebra_ui::style::text::Text::Dabger);
        let content_col = Column::new().push(title).push(message);
        let content = Container::new(content_col)
            .height(250)
            .width(450)
            .style(zebra_ui::style::container::Container::SecondaryRoundedBox);
        let row = Row::new()
            .height(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(content);
        let col = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(row);

        Container::new(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl ErrorPage {
    pub fn from(message: String) -> Self {
        Self { message }
    }
}
