//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use iced::widget::{Column, Container, Row, Space, Text};
use iced::{Command, Element, Length, Subscription};
use zebra_lib::{core::core::Core, errors::ZebraErrors};

use crate::components::home_nav_bar::{NavBar, NavRoute, LINE_ALFA_CHANNEL};
use crate::components::select_list;
use crate::components::smart_input::SmartInput;
use crate::config::categories::Categories;
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
    categories: Vec<select_list::SelectListField<Categories>>,
    selected: Categories,
}

#[derive(Debug, Clone)]
pub enum AddRecordPageMessage {
    RouteGen,
    RouteHome,
    RouteSettings,
    HanldeSelectCategories(usize),
}

impl Page for AddRecordPage {
    type Message = AddRecordPageMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let categories = vec![
            select_list::SelectListField {
                text: String::from("test"),
                value: Categories::Login,
            },
            select_list::SelectListField {
                text: String::from("fsdfds"),
                value: Categories::Login,
            },
            select_list::SelectListField {
                text: String::from("fdgf89h"),
                value: Categories::Login,
            },
        ];
        let selected = Categories::Login;

        Ok(Self {
            core,
            categories,
            selected,
        })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<GlobalMessage> {
        match message {
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
            AddRecordPageMessage::HanldeSelectCategories(index) => {
                dbg!(index);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let login_form = self.login_form();
        let vline = zebra_ui::components::line::Linear::new()
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .style(zebra_ui::styles::line::line_secondary)
            .alfa(LINE_ALFA_CHANNEL);
        let categories = select_list::SelectList::from(&self.categories)
            .on_select(AddRecordPageMessage::HanldeSelectCategories)
            .set_gap(5)
            .set_line_gap(10)
            .set_field_padding(8);
        let categories = Container::new(categories);
        let left_col = Column::new()
            .height(Length::Fill)
            .width(200)
            .push(Space::new(0, 5))
            .push(categories)
            .push(Space::new(0, 5));
        let content_row = Row::new().push(left_col).push(vline).push(login_form);
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
        let title = Text::new(t!("add_form_title"))
            .size(16)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);

        let name_input = SmartInput::new();
        let name_input = Container::new(name_input);

        let username_input = SmartInput::new();
        let username_input = Container::new(username_input);

        let password_input = SmartInput::new();
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
