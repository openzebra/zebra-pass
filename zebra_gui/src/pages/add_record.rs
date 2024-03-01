//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use crate::rust_i18n::t;
use iced::widget::{text_input, Space};
use iced::{alignment, Command, Length, Subscription};
use zebra_lib::{core::core::Core, errors::ZebraErrors};
use zebra_ui::widget::*;

use crate::components::home_nav_bar::{NavBar, NavRoute, LINE_ALFA_CHANNEL};
use crate::gui::{GlobalMessage, Routers};

use super::error::ErrorPage;
use super::gen::Generator;
use super::home::Home;
use super::settings::Settings;
use super::Page;

#[derive(Debug)]
pub struct AddRecordPage {
    core: Arc<Mutex<Core>>,
}

#[derive(Debug, Clone)]
pub enum AddRecordPageMessage {
    RouteGen,
    RouteHome,
    RouteSettings,
    HanldeInputName(String),
}

impl Page for AddRecordPage {
    type Message = AddRecordPageMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        Ok(Self { core })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<GlobalMessage> {
        match message {
            AddRecordPageMessage::HanldeInputName(v) => {
                dbg!(v);

                Command::none()
            }

            AddRecordPageMessage::RouteGen => match Generator::new(Arc::clone(&self.core)) {
                Ok(gen) => {
                    let route = Routers::Generator(gen);

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },
            AddRecordPageMessage::RouteSettings => match Settings::new(Arc::clone(&self.core)) {
                Ok(settings) => {
                    let route = Routers::Settings(settings);

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },
            AddRecordPageMessage::RouteHome => match Home::new(Arc::clone(&self.core)) {
                Ok(home) => {
                    let route = Routers::Home(home);

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
        let content = self.add_form();

        NavBar::<Self::Message>::new()
            .set_route(NavRoute::None)
            .on_gen(AddRecordPageMessage::RouteGen)
            .on_settings(AddRecordPageMessage::RouteSettings)
            .on_home(AddRecordPageMessage::RouteHome)
            .view(content)
            .into()
    }
}

impl AddRecordPage {
    pub fn input(&self) -> Container<AddRecordPageMessage> {
        let label = Text::new("name").size(12);
        let input = text_input("", "")
            .size(14)
            .padding(4)
            .secure(true)
            .on_input(AddRecordPageMessage::HanldeInputName)
            .style(zebra_ui::style::text_input::TextInput::Transparent);
        let col = Column::new().push(label).push(input);
        let copy_btn = Button::new(zebra_ui::image::copy_icon().height(25).width(25))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent);
        let reload_btn = Button::new(zebra_ui::image::reload_icon().height(30).width(30))
            .padding(0)
            .style(zebra_ui::style::button::Button::Transparent);
        let row = Row::new()
            .align_items(iced::Alignment::Center)
            .push(col)
            .push(copy_btn)
            .push(reload_btn);

        Container::new(row)
            .padding(3)
            .style(zebra_ui::style::container::Container::SecondaryRoundedBox)
    }

    pub fn add_form(&self) -> Container<AddRecordPageMessage> {
        let vline = zebra_ui::components::line::Line::new()
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .alfa(LINE_ALFA_CHANNEL)
            .style(zebra_ui::components::line::LineStyleSheet::Secondary);
        let left_search_col = Column::new().height(Length::Fill).width(200);
        let row = Row::new()
            .push(left_search_col)
            .push(vline)
            .push(self.input());

        Container::new(row)
    }
}
