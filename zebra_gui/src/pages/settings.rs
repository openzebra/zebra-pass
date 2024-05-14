//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use iced::widget::{Column, Container, Row};
use iced::{Command, Element, Length, Subscription};
use zebra_lib::{core::Core, errors::ZebraErrors};

use crate::components::home_nav_bar::{NavBar, NavRoute};
use crate::gui::{GlobalMessage, Routers};

use super::add_record::AddRecordPage;
use super::error::ErrorPage;
use super::gen::Generator;
use super::home::Home;
use super::Page;

#[derive(Debug)]
pub struct Settings {
    core: Arc<Mutex<Core>>,
}

#[derive(Debug, Clone, Copy)]
pub enum SettingsMessage {
    RouteHome,
    RouteGen,
    AddRecord,
}

impl Page for Settings {
    type Message = SettingsMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        Ok(Self { core })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<GlobalMessage> {
        match message {
            SettingsMessage::RouteHome => match Home::new(Arc::clone(&self.core)) {
                Ok(home) => {
                    let route = Routers::Home(home);

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },
            SettingsMessage::RouteGen => match Generator::new(Arc::clone(&self.core)) {
                Ok(gen) => {
                    let route = Routers::Generator(gen);

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },

            SettingsMessage::AddRecord => match AddRecordPage::new(Arc::clone(&self.core)) {
                Ok(add_record) => {
                    let route = Routers::AddRecord(add_record);

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let records = &self.core.lock().unwrap().data;
        let content = Container::new(if records.is_empty() {
            self.view_no_records()
        } else {
            self.view_records()
        });

        NavBar::<Self::Message>::new()
            .set_route(NavRoute::Settings)
            .on_home(SettingsMessage::RouteHome)
            .on_gen(SettingsMessage::RouteGen)
            .on_add(SettingsMessage::AddRecord)
            .view(content)
            .into()
    }
}

impl Settings {
    pub fn view_no_records(&self) -> Row<SettingsMessage> {
        Row::new()
    }

    pub fn view_records(&self) -> Row<SettingsMessage> {
        let vline = zebra_ui::components::line::Linear::new()
            .width(Length::Fixed(1.0))
            .height(Length::Fill);
        let left_search_col = Column::new().height(Length::Fill).width(200);

        Row::new().push(left_search_col).push(vline)
    }
}
