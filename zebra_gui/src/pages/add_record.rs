//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use iced::widget::{Column, Container, Row, Text};
use iced::{Command, Element, Length, Subscription};
use zebra_lib::{core::core::Core, errors::ZebraErrors};

use crate::components::add_login::AddLogin;
use crate::components::home_nav_bar::{NavBar, NavRoute, LINE_ALFA_CHANNEL};
use crate::components::select_list;
use crate::gui::{GlobalMessage, Routers};
use crate::rust_i18n::t;
use zebra_lib::core::record;

use super::error::ErrorPage;
use super::gen::Generator;
use super::home::Home;
use super::settings::Settings;
use super::Page;

#[derive(Debug)]
pub struct AddRecordPage {
    core: Arc<Mutex<Core>>,
    categories: Vec<select_list::SelectListField<record::Categories>>,
    selected: record::Categories,
    selected_index: usize,
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
        let selected_index = 0;
        let categories = vec![
            select_list::SelectListField {
                text: t!(&format!(
                    "item_{}",
                    record::Categories::Login(Default::default())
                )),
                value: record::Categories::Login(record::Element {
                    fields: vec![
                        record::Item {
                            title: t!("placeholder_name"),
                            value: String::new(),
                            hide: false,
                            copy: true,
                        },
                        record::Item {
                            title: t!("placeholder_domain"),
                            value: String::new(),
                            hide: false,
                            copy: true,
                        },
                        record::Item {
                            title: t!("placeholder_username"),
                            value: String::new(),
                            hide: false,
                            copy: true,
                        },
                        record::Item {
                            title: t!("placeholder_email"),
                            value: String::new(),
                            hide: false,
                            copy: true,
                        },
                        record::Item {
                            title: t!("placeholder_password"),
                            value: String::new(),
                            hide: true,
                            copy: true,
                        },
                    ],
                    extra_fields: Vec::new(),
                    ..Default::default()
                }),
            },
            // select_list::SelectListField {
            //     text: t!(&format!(
            //         "item_{}",
            //         Categories::CryptoWallet(Default::default())
            //     )),
            //     value: Categories::CryptoWallet,
            // },
            // select_list::SelectListField {
            //     text: t!(&format!(
            //         "item_{}",
            //         Categories::CreditCard(Default::default())
            //     )),
            //     value: Categories::CreditCard,
            // },
            // select_list::SelectListField {
            //     text: t!(&format!("item_{}", Categories::Identity)),
            //     value: Categories::Identity,
            // },
            // select_list::SelectListField {
            //     text: t!(&format!("item_{}", Categories::BankAccount)),
            //     value: Categories::BankAccount,
            // },
            // select_list::SelectListField {
            //     text: t!(&format!("item_{}", Categories::EmailAccount)),
            //     value: Categories::EmailAccount,
            // },
            // select_list::SelectListField {
            //     text: t!(&format!("item_{}", Categories::Passport)),
            //     value: Categories::Passport,
            // },
            // select_list::SelectListField {
            //     text: t!(&format!("item_{}", Categories::DriverLicense)),
            //     value: Categories::DriverLicense,
            // },
            // select_list::SelectListField {
            //     text: t!(&format!("item_{}", Categories::WifiPassword)),
            //     value: Categories::WifiPassword,
            // },
            // select_list::SelectListField {
            //     text: t!(&format!("item_{}", Categories::Other)),
            //     value: Categories::Other,
            // },
        ];
        let selected = record::Categories::Login(Default::default());

        Ok(Self {
            selected_index,
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
                match self.categories.get(index) {
                    Some(v) => {
                        self.selected_index = index;
                        self.selected = v.value.clone();
                    }
                    None => {}
                };

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let vline = zebra_ui::components::line::Linear::new()
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .style(zebra_ui::styles::line::line_secondary)
            .alfa(LINE_ALFA_CHANNEL);
        let categories = select_list::SelectList::from(&self.categories)
            .on_select(AddRecordPageMessage::HanldeSelectCategories)
            .set_selected_index(self.selected_index)
            .set_text_horizontal_alignmen(iced::alignment::Horizontal::Left)
            .set_line_gap(10)
            .set_field_padding(8);
        let categories = Container::new(categories);
        let left_col = Column::new()
            .height(Length::Fill)
            .width(200)
            .push(categories);
        let form = match self.selected {
            record::Categories::Login(_) => {
                let f = AddLogin::new().set_title(t!(&format!(
                    "item_{}",
                    record::Categories::Login(Default::default())
                )));

                Container::new(f)
            }
            _ => {
                let ctx = Text::new("not implemented yet");

                Container::new(ctx)
            }
        };
        let content_row = Row::new().push(left_col).push(vline).push(form);
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
