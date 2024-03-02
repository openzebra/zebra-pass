//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use iced::{Command, Length, Subscription};
use zebra_lib::{core::core::Core, errors::ZebraErrors};
use zebra_ui::widget::*;

use crate::components::home_nav_bar::{NavBar, NavRoute, LINE_ALFA_CHANNEL};
use crate::components::smart_input::{SmartInput, SmartInputState};
use crate::gui::{GlobalMessage, Routers};
use crate::rust_i18n::t;

use super::error::ErrorPage;
use super::gen::Generator;
use super::home::Home;
use super::settings::Settings;
use super::Page;

#[derive(Debug)]
pub struct AddRecordPage {
    core: Arc<Mutex<Core>>,
    name_input_state: Arc<Mutex<SmartInputState>>,
    username_input_state: Arc<Mutex<SmartInputState>>,
    password_input_state: Arc<Mutex<SmartInputState>>,
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
        let name_input_state = Arc::new(Mutex::new(SmartInputState {
            secured: false,
            placeholder: String::new(),
            value: String::new(),
            label: t!("add_form_name"),
        }));
        let username_input_state = Arc::new(Mutex::new(SmartInputState {
            secured: false,
            placeholder: String::new(),
            value: String::new(),
            label: t!("add_form_username"),
        }));
        let password_input_state = Arc::new(Mutex::new(SmartInputState {
            secured: true,
            placeholder: String::new(),
            value: String::new(),
            label: t!("add_form_password"),
        }));

        Ok(Self {
            core,
            name_input_state,
            username_input_state,
            password_input_state,
        })
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
        let login_form = self.login_form();
        let vline = zebra_ui::components::line::Line::new()
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .alfa(LINE_ALFA_CHANNEL)
            .style(zebra_ui::components::line::LineStyleSheet::Secondary);
        let left_search_col = Column::new().height(Length::Fill).width(200);
        let content_row = Row::new()
            .push(left_search_col)
            .push(vline)
            .push(login_form);
        let main_container = Container::new(content_row).width(Length::Fill);

        NavBar::<Self::Message>::new()
            .set_route(NavRoute::None)
            .on_gen(AddRecordPageMessage::RouteGen)
            .on_settings(AddRecordPageMessage::RouteSettings)
            .on_home(AddRecordPageMessage::RouteHome)
            .view(main_container)
            .into()
    }
}

impl AddRecordPage {
    pub fn login_form(&self) -> Container<AddRecordPageMessage> {
        let title = Text::new("ITEM INFORMATION")
            .size(16)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);

        let name_input = SmartInput::new(Arc::clone(&self.name_input_state));
        let name_input = Container::new(name_input);

        let username_input = SmartInput::new(Arc::clone(&self.username_input_state));
        let username_input = Container::new(username_input);

        let password_input = SmartInput::new(Arc::clone(&self.password_input_state));
        let password_input = Container::new(password_input);

        let main_col = Column::new()
            .padding(16)
            .spacing(8)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(title)
            .push(name_input)
            .push(username_input)
            .push(password_input);

        Container::new(main_col)
    }
}
