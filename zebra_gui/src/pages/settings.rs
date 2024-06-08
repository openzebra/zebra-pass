//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use iced::widget::{Column, Container, Row, Text};
use iced::{Command, Element, Length, Subscription};
use zebra_lib::{core::Core, errors::ZebraErrors};

use crate::components::home_nav_bar::{NavBar, NavRoute, LINE_ALFA_CHANNEL};
use crate::components::select_list;
use crate::gui::{GlobalMessage, Routers};
use crate::rust_i18n::t;

use super::add_record::AddRecordPage;
use super::error::ErrorPage;
use super::gen::Generator;
use super::home::Home;
use super::Page;

#[derive(Debug, Clone)]
enum SettingsOptions {
    Profile,
    General,
    Crypto,
    Advanced,
    Network,
}

#[derive(Debug)]
pub struct Settings {
    core: Arc<Mutex<Core>>,
    selected_index: usize,
    selected_option: SettingsOptions,
    options_list: Vec<select_list::SelectListField<SettingsOptions>>,
}

#[derive(Debug, Clone, Copy)]
pub enum SettingsMessage {
    RouteHome,
    RouteGen,
    AddRecord,
    HanldeSelectOption(usize),
}

impl Page for Settings {
    type Message = SettingsMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let options_list: Vec<select_list::SelectListField<SettingsOptions>> = vec![
            select_list::SelectListField {
                text: t!("profile").to_string(),
                value: SettingsOptions::Profile,
            },
            select_list::SelectListField {
                text: t!("general").to_string(),
                value: SettingsOptions::General,
            },
            select_list::SelectListField {
                text: t!("advanced").to_string(),
                value: SettingsOptions::Advanced,
            },
            select_list::SelectListField {
                text: t!("crypto").to_string(),
                value: SettingsOptions::Crypto,
            },
            select_list::SelectListField {
                text: t!("network").to_string(),
                value: SettingsOptions::Network,
            },
        ];
        Ok(Self {
            core,
            options_list,
            selected_index: 0,
            selected_option: SettingsOptions::Profile,
        })
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
            SettingsMessage::HanldeSelectOption(index) => {
                if let Some(v) = self.options_list.get(index) {
                    self.selected_index = index;
                    self.selected_option = v.value.clone();
                }

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let categories = select_list::SelectList::from(&self.options_list)
            .on_select(SettingsMessage::HanldeSelectOption)
            .set_selected_index(self.selected_index)
            .set_text_horizontal_alignmen(iced::alignment::Horizontal::Left)
            .set_line_gap(10)
            .set_field_padding(8);
        let categories = Container::new(categories);
        let vline = zebra_ui::components::line::Linear::new()
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .style(zebra_ui::styles::line::line_secondary)
            .alfa(LINE_ALFA_CHANNEL);
        let left_search_col = Column::new()
            .height(Length::Fill)
            .width(200)
            .push(categories);
        let page = match self.selected_option {
            SettingsOptions::Profile => self.view_profile(),
            SettingsOptions::Network => self.view_network(),
            SettingsOptions::Advanced => self.view_advanced(),
            SettingsOptions::General => self.view_general(),
            SettingsOptions::Crypto => self.view_crypto(),
        };
        let row = Row::new().push(left_search_col).push(vline).push(page);
        let content = Container::new(row);

        NavBar::<Self::Message>::new()
            .set_route(NavRoute::Settings)
            .on_home(&SettingsMessage::RouteHome)
            .on_gen(&SettingsMessage::RouteGen)
            .on_add(&SettingsMessage::AddRecord)
            .view(content)
            .into()
    }
}

impl Settings {
    pub fn view_profile(&self) -> Container<SettingsMessage> {
        let title = Text::new(&self.options_list[self.selected_index].text);
        // profile info, export secret phrase, change password,
        let main_col = Column::new().push(title);

        Container::new(main_col)
    }

    pub fn view_general(&self) -> Container<SettingsMessage> {
        // theme, locale.
        let main_col = Column::new();

        Container::new(main_col)
    }

    pub fn view_advanced(&self) -> Container<SettingsMessage> {
        let main_col = Column::new();

        Container::new(main_col)
    }

    pub fn view_crypto(&self) -> Container<SettingsMessage> {
        let main_col = Column::new();

        Container::new(main_col)
    }

    pub fn view_network(&self) -> Container<SettingsMessage> {
        let main_col = Column::new();

        Container::new(main_col)
    }
}
