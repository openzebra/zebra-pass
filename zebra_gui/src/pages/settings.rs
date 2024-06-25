//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex, MutexGuard};

use iced::widget::{Button, Column, Container, Row, Space, Text};
use iced::{Command, Element, Length, Subscription};
use iced::{Renderer, Theme};
use zebra_lib::{core::Core, errors::ZebraErrors};

use dirs;
use rfd::FileDialog;

use crate::components::home_nav_bar::{NavBar, NavRoute, LINE_ALFA_CHANNEL};
use crate::components::modal::Modal;
use crate::components::{profile_view::ProfileViewForm, select_list};
use crate::gui::{GlobalMessage, Routers};
use crate::rust_i18n::t;

use super::add_record::AddRecordPage;
use super::error::ErrorPage;
use super::gen::Generator;
use super::home::Home;
use super::Page;

const MAIN_PADDING: f32 = 16.0;
const ITEM_PADDING: f32 = 8.0;

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
    remove_modal: bool,
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
    EditEmail(String),
    Remove,
    RemoveModal,
    ExportRecords,
    ExportDatabase,
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
            remove_modal: false,
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
            SettingsMessage::EditEmail(value) => {
                let _ = self.core.lock().map(|mut core| core.set_email(value));
                Command::none()
            }
            SettingsMessage::Remove => {
                //
                Command::none()
            }
            SettingsMessage::ExportRecords => {
                //
                Command::none()
            }
            SettingsMessage::ExportDatabase => {
                let error_msg;

                if let Some(home_dir) = dirs::home_dir() {
                    let files = FileDialog::new()
                        .set_file_name("zebrapass.zebra")
                        .set_directory(home_dir)
                        .save_file();

                    if let Some(path) = files {
                        match self.core.lock() {
                            Ok(core) => {
                                match core.export_to_file(&path) {
                                    Ok(_) => {
                                        return Command::none();
                                    }
                                    Err(e) => {
                                        error_msg = e.to_string();
                                    }
                                };
                            }
                            Err(e) => {
                                error_msg = e.to_string();
                            }
                        }
                    } else {
                        error_msg = t!("cannot_load_save_dir").to_string();
                    }
                } else {
                    error_msg = t!("cannot_get_access_home_dir").to_string();
                }

                let route = Routers::ErrorPage(ErrorPage::from(error_msg));
                Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
            }
            SettingsMessage::RemoveModal => {
                self.remove_modal = !self.remove_modal;
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

        let page = if let Ok(core) = self.core.lock() {
            match self.selected_option {
                SettingsOptions::Profile => self.view_profile(core),
                SettingsOptions::Network => self.view_network(),
                SettingsOptions::Advanced => self.view_advanced(),
                SettingsOptions::General => self.view_general(),
                SettingsOptions::Crypto => self.view_crypto(),
            }
        } else {
            self.view_error()
        };

        let row = Row::new().push(left_search_col).push(vline).push(page);
        let content = if self.remove_modal {
            let modal =
                Modal::new(row, self.view_remove_modal()).on_blur(SettingsMessage::RemoveModal);
            Container::new(modal)
        } else {
            Container::new(row)
        };

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
    pub fn view_remove_modal<'a>(&self) -> Container<'a, SettingsMessage, Theme, Renderer> {
        let close_btn = Button::new(
            zebra_ui::image::close_icon()
                .style(zebra_ui::styles::svg::primary_hover)
                .height(30)
                .width(30),
        )
        .padding(0)
        .style(zebra_ui::styles::button::transparent)
        .on_press(SettingsMessage::RemoveModal);
        let close_btn = Column::new()
            .push(close_btn)
            .width(Length::Fill)
            .align_items(iced::Alignment::End);
        let row_header = Row::new().padding(8).push(close_btn).width(Length::Fill);

        let save_btn = Button::new(
            Text::new("")
                .size(MAIN_PADDING)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .style(zebra_ui::styles::button::outline_primary)
        .padding(ITEM_PADDING)
        .on_press(SettingsMessage::RemoveModal);

        let main_modal_col = Column::new()
            .push(row_header)
            .push(save_btn)
            .push(Space::new(0, ITEM_PADDING))
            .padding(ITEM_PADDING)
            .align_items(iced::Alignment::Center);

        Container::new(main_modal_col)
            .width(400)
            .style(zebra_ui::styles::container::primary_bordered_modal)
    }

    pub fn view_profile(&self, core: MutexGuard<Core>) -> Container<SettingsMessage> {
        let title = Text::new(&self.options_list[self.selected_index].text)
            .size(24)
            .horizontal_alignment(iced::alignment::Horizontal::Left)
            .width(Length::Fill);
        let profile_view = ProfileViewForm::new()
            .set_data_size(core.get_data_size())
            .set_records_len(core.data.len())
            .set_storage_version(core.get_storage_version())
            .set_email(core.state.email.clone().unwrap_or_default())
            .set_address(core.state.address.clone())
            .set_data_dir_path(core.get_data_dir().to_string_lossy().to_string().into())
            .on_copy(SettingsMessage::CopyValue)
            .on_edit_email(SettingsMessage::EditEmail)
            .on_export_database(SettingsMessage::ExportDatabase)
            .set_main_padding(MAIN_PADDING)
            .set_item_padding(ITEM_PADDING);
        let profile_view = Container::new(profile_view);

        let remove_button = Button::new(Text::new(t!("remove")).size(14))
            .padding(0)
            .on_press(SettingsMessage::RemoveModal)
            .style(zebra_ui::styles::button::ref_danger);
        let remove_button_row = Row::new().width(Length::Fill).push(remove_button);

        let main_col = Column::new()
            .align_items(iced::Alignment::Center)
            .padding(MAIN_PADDING)
            .push(title)
            .push(Space::new(0, MAIN_PADDING))
            .push(profile_view)
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

    pub fn view_error(&self) -> Container<SettingsMessage> {
        let main_col = Column::new();

        Container::new(main_col)
    }
}
