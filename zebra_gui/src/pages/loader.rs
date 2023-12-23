//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use crate::rust_i18n::t;
use iced::{
    widget::{text, Space},
    Alignment, Command, Length, Subscription,
};
use zebra_lib::{core::core::Core, errors::ZebraErrors};
use zebra_ui::{components::circular::Circular, widget::*};

use crate::gui::{GlobalMessage, Routers};

use super::{locale::Locale, Page};

#[derive(Debug)]
pub struct Loader {
    error: Option<String>,
    core: Arc<Mutex<Core>>,
}

#[derive(Debug, Clone, Copy)]
pub enum LoadMessage {
    Synced,
}

impl Page for Loader {
    type Message = LoadMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        Ok(Self { error: None, core })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<GlobalMessage> {
        match message {
            LoadMessage::Synced => {
                let locale = Locale::new(Arc::clone(&self.core)).unwrap();
                let route = Routers::Locale(locale);
                Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let message = match &self.error {
            Some(err) => text(err).size(25),
            None => text(t!("loading")).size(25),
        }
        .horizontal_alignment(iced::alignment::Horizontal::Center);
        let spiner = Circular::new();
        let col_loading = Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(Space::new(0, 100))
            .push(message)
            .push(spiner);
        let main_row = Row::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .push(col_loading);

        Container::new(main_row)
            .height(Length::Fill)
            .width(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}