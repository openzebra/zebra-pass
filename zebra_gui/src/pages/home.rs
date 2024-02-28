//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use crate::rust_i18n::t;
use iced::widget::Space;
use iced::{Command, Length, Subscription};
use zebra_lib::{core::core::Core, errors::ZebraErrors};
use zebra_ui::widget::*;

use crate::components::home_nav_bar::{NavBar, NavRoute, LINE_ALFA_CHANNEL};
use crate::gui::{GlobalMessage, Routers};

use super::error::ErrorPage;
use super::gen::Generator;
use super::settings::Settings;
use super::Page;

#[derive(Debug)]
pub struct Home {
    core: Arc<Mutex<Core>>,
}

#[derive(Debug, Clone, Copy)]
pub enum HomeMessage {
    RouteGen,
    RouteSettings,
}

impl Page for Home {
    type Message = HomeMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        Ok(Self { core })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<GlobalMessage> {
        match message {
            HomeMessage::RouteGen => match Generator::new(Arc::clone(&self.core)) {
                Ok(gen) => {
                    let route = Routers::Generator(gen);

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },
            HomeMessage::RouteSettings => match Settings::new(Arc::clone(&self.core)) {
                Ok(settings) => {
                    let route = Routers::Settings(settings);

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
        let records = &self.core.lock().unwrap().data; // TODO: remove unwrap..
        let content = Container::new(if records.is_empty() {
            self.view_options()
        } else {
            self.view_records()
        });

        NavBar::<Self::Message>::new()
            .set_route(NavRoute::Home)
            .on_gen(HomeMessage::RouteGen)
            .on_settings(HomeMessage::RouteSettings)
            .view(content)
            .into()
    }
}

impl Home {
    pub fn view_options(&self) -> Container<HomeMessage> {
        let title = Text::new(t!("no_records_title")).size(21);
        let row = Row::new()
            .align_items(iced::Alignment::Start)
            .padding(16)
            .push(title);

        let add_btn = Button::new(zebra_ui::image::add_icon().height(70).width(70))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent);
        let options_row = Row::new()
            .align_items(iced::Alignment::Center)
            .height(Length::Fill)
            .push(add_btn);
        let options = Container::new(options_row)
            .padding(8)
            .height(250)
            .width(400)
            .style(zebra_ui::style::container::Container::Bordered);
        let col = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(row)
            .push(options);

        Container::new(col).width(Length::Fill).height(Length::Fill)
    }

    pub fn view_records(&self) -> Container<HomeMessage> {
        let vline = zebra_ui::components::line::Line::new()
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .alfa(LINE_ALFA_CHANNEL)
            .style(zebra_ui::components::line::LineStyleSheet::Secondary);
        let left_search_col = Column::new().height(Length::Fill).width(200);
        let row = Row::new().push(left_search_col).push(vline);

        Container::new(row)
    }
}
