//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use crate::rust_i18n::t;
use iced::widget::{Button, Column, Container, Row, Space, Text};
use iced::{alignment, window, Element, Length, Subscription};
use zebra_lib::{core::core::Core, errors::ZebraErrors};

use crate::gui::GlobalMessage;

use super::Page;

#[derive(Debug)]
pub struct ErrorPage {
    message: String,
}

#[derive(Debug, Clone, Copy)]
pub enum ErrorPageMessage {
    Exit,
}

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
        match message {
            ErrorPageMessage::Exit => window::close(window::Id::MAIN),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let title = Text::new(t!("error_page_title"))
            // .style(zebra_ui::style::text::Text::Info);
            .size(18);
        let message = Text::new(&self.message)
            // .style(zebra_ui::style::text::Text::Dabger)
            .size(14);
        let exit_btn = Button::new(
            Text::new("exit")
                .size(14)
                .horizontal_alignment(alignment::Horizontal::Center),
        )
        .padding(8)
        .width(120)
        // .style(zebra_ui::style::button::Button::Primary)
        .on_press(ErrorPageMessage::Exit);
        let content_col = Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(zebra_ui::image::bad_zebra_view().width(160).height(120))
            .push(title)
            .push(message)
            .push(Space::new(0, 16))
            .push(exit_btn);
        let content = Container::new(content_col)
            .height(250)
            // .style(zebra_ui::style::container::Container::WeekBorder)
            .width(450);
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
