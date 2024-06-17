//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
};

use iced::{
    widget::{Button, Column, Container, Row, Space, Text},
    Padding, Theme,
};
use iced::{Command, Element, Length, Subscription};
use zebra_lib::{core::Core, errors::ZebraErrors};

use crate::components::select_list;
use crate::components::{
    home_nav_bar::{NavBar, NavRoute, LINE_ALFA_CHANNEL},
    smart_field::SmartFields,
};
use crate::gui::{GlobalMessage, Routers};
use crate::rust_i18n::t;
use zebra_ui::components::line::Linear;

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

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    RouteHome,
    RouteGen,
    AddRecord,
    HanldeSelectOption(usize),
    CopyValue(String),
    EditEmail,
    Remove,
    Export,
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
            SettingsMessage::CopyValue(value) => iced::clipboard::write::<GlobalMessage>(value),
            SettingsMessage::EditEmail => {
                //
                Command::none()
            }
            SettingsMessage::Remove => {
                //
                Command::none()
            }
            SettingsMessage::Export => {
                //
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
    pub fn view_hline(&self) -> Linear<Theme> {
        Linear::new()
            .height(Length::Fixed(0.5))
            .width(Length::Fill)
            .style(zebra_ui::styles::line::line_secondary)
            .alfa(LINE_ALFA_CHANNEL)
    }

    pub fn view_profile(&self) -> Container<SettingsMessage> {
        const MAIN_PADDING: f32 = 16.0;
        const ITEM_PADDING: f32 = 8.0;

        // TODO: remove unwerap.
        let core = self.core.lock().unwrap();
        let title = Text::new(&self.options_list[self.selected_index].text)
            .size(24)
            .horizontal_alignment(iced::alignment::Horizontal::Left)
            .width(Length::Fill);

        let address = SmartFields::new()
            .set_label(t!("address"))
            .set_padding(ITEM_PADDING)
            .set_truncate(true)
            .on_copy(SettingsMessage::CopyValue)
            .set_value(core.state.address.clone());
        let address = Container::new(address);

        let email = SmartFields::new()
            .set_label(t!("email"))
            .set_padding(ITEM_PADDING)
            .on_copy(SettingsMessage::CopyValue)
            .on_edit(SettingsMessage::EditEmail)
            .set_value(core.state.email.clone().unwrap_or(t!("not_set")));
        let email = Container::new(email);

        let records = SmartFields::new()
            .set_label(t!("amount_of_records"))
            .set_padding(ITEM_PADDING)
            .on_export(SettingsMessage::Export)
            .set_value(core.data.len().to_string().into());
        let records = Container::new(records);

        let data_dir_path = core.get_data_dir().to_string_lossy().to_string();
        let data_dir = SmartFields::new()
            .set_label(t!("database_path"))
            .set_padding(ITEM_PADDING)
            .on_copy(SettingsMessage::CopyValue)
            .set_value(Cow::Owned(data_dir_path));
        let data_dir = Container::new(data_dir);

        let data_size = SmartFields::new()
            .set_label(t!("database_size"))
            .set_padding(ITEM_PADDING)
            .set_value(format!("{} bytes", core.get_data_size()).into());
        let data_size = Container::new(data_size);

        let storage_version = SmartFields::new()
            .set_label(t!("storage_version"))
            .set_padding(ITEM_PADDING)
            .set_value(format!("V{}", core.get_storage_version()).into());
        let storage_version = Container::new(storage_version);

        let remove_button = Button::new(Text::new(t!("remove")).size(14))
            .padding(0)
            .on_press(SettingsMessage::Remove)
            .style(zebra_ui::styles::button::ref_danger);
        let remove_button_row = Row::new().width(Length::Fill).push(remove_button);

        let border_col = Column::new()
            .push(address)
            .push(self.view_hline())
            .push(email)
            .push(self.view_hline())
            .push(data_dir)
            .push(self.view_hline())
            .push(records)
            .push(self.view_hline())
            .push(data_size)
            .push(self.view_hline())
            .push(storage_version);
        let border = Container::new(border_col)
            .padding(Padding {
                left: ITEM_PADDING,
                right: ITEM_PADDING,
                top: 0.0,
                bottom: 0.0,
            })
            .width(Length::Fill)
            .style(zebra_ui::styles::container::primary_bordered);
        let main_col = Column::new()
            .align_items(iced::Alignment::Center)
            .padding(MAIN_PADDING)
            .push(title)
            .push(Space::new(0, MAIN_PADDING))
            .push(border)
            .push(Space::new(0, MAIN_PADDING))
            .push(remove_button_row);

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
