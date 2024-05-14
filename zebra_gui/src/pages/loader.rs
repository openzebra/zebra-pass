//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use crate::rust_i18n::t;
use iced::widget::{Column, Container, Row, Space, Text};
use iced::{Alignment, Command, Element, Length, Subscription};
use zebra_lib::{core::Core, errors::ZebraErrors};
use zebra_ui::components::circular::Circular;

use crate::gui::{GlobalMessage, Routers};

use super::{error::ErrorPage, home::Home, locale::Locale, lock::Lock, Page};

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
                // TODO: remove unwrap.
                let core = self.core.lock().unwrap();

                if !core.state.inited {
                    drop(core);

                    return match Locale::new(Arc::clone(&self.core)) {
                        Ok(locale) => {
                            let route = Routers::Locale(locale);
                            Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                        }
                        Err(e) => {
                            let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));
                            Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                        }
                    };
                }

                if core.is_unlock() {
                    drop(core);
                    match Home::new(Arc::clone(&self.core)) {
                        Ok(home) => {
                            let route = Routers::Home(home);
                            Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                        }
                        Err(e) => {
                            let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));
                            Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                        }
                    }
                } else {
                    drop(core);
                    match Lock::new(Arc::clone(&self.core)) {
                        Ok(lock) => {
                            let route = Routers::Lock(lock);
                            Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                        }
                        Err(e) => {
                            let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));
                            Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                        }
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let message = match &self.error {
            Some(err) => Text::new(err).size(25),
            None => Text::new(t!("loading")).size(25),
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
