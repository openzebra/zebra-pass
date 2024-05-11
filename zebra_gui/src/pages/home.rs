//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use crate::components::add_record_from::AddRecordForm;
use crate::components::select_list;
use crate::rust_i18n::t;
use iced::widget::{Button, Column, Container, Row, Text};
use iced::{alignment, Command, Element, Length, Subscription};
use zebra_lib::core::record;
use zebra_lib::{core::core::Core, errors::ZebraErrors};

use crate::components::home_nav_bar::{NavBar, NavRoute, LINE_ALFA_CHANNEL};
use crate::gui::{GlobalMessage, Routers};

use super::add_record::AddRecordPage;
use super::error::ErrorPage;
use super::gen::Generator;
use super::settings::Settings;
use super::Page;

#[derive(Debug)]
pub struct Home {
    core: Arc<Mutex<Core>>,
    read_only: bool,
    selected_index: usize,
    categories_list: Vec<select_list::SelectListField<record::Categories>>,
}

#[derive(Debug, Clone)]
pub enum HomeMessage {
    RouteGen,
    RouteSettings,
    AddRecord,
    Copy(String),
    HanldeSelectCategories(usize),
}

impl Page for Home {
    type Message = HomeMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let records = match core.lock() {
            // TODO: this is bad praticle! the big array copy in ram, need rework with pointers.
            Ok(state) => state.data.clone(),
            Err(_) => Vec::new(),
        };
        let categories_list: Vec<select_list::SelectListField<record::Categories>> = records
            .iter()
            .map(|element| select_list::SelectListField {
                text: element.get_value().name.clone(),
                value: element.clone(),
            })
            .collect();

        Ok(Self {
            core,
            categories_list,
            read_only: true,
            selected_index: 0,
        })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<GlobalMessage> {
        match message {
            HomeMessage::Copy(value) => iced::clipboard::write::<GlobalMessage>(value),
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
            HomeMessage::AddRecord => match AddRecordPage::new(Arc::clone(&self.core)) {
                Ok(add_record) => {
                    let route = Routers::AddRecord(add_record);

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },
            HomeMessage::HanldeSelectCategories(index) => {
                self.selected_index = index;

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let records = &self.core.lock().unwrap(); // TODO: remove unwrap..
        let content = Container::new(if records.data.is_empty() {
            self.view_options()
        } else {
            self.view_records()
        });

        NavBar::<Self::Message>::new()
            .set_route(NavRoute::Home)
            .on_gen(HomeMessage::RouteGen)
            .on_settings(HomeMessage::RouteSettings)
            .on_add(HomeMessage::AddRecord)
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

        // TODO: add more options for import..
        let add_btn = Button::new(
            zebra_ui::image::add_icon()
                .style(zebra_ui::styles::svg::primary_hover)
                .height(70)
                .width(70),
        )
        .padding(0)
        .style(zebra_ui::styles::button::transparent)
        .on_press(HomeMessage::AddRecord);
        let options_row = Row::new()
            .align_items(iced::Alignment::Center)
            .height(Length::Fill)
            .push(add_btn);
        let options = Container::new(options_row)
            .align_x(alignment::Horizontal::Center)
            .padding(8)
            .height(250)
            .style(zebra_ui::styles::container::primary_bordered)
            .width(400);
        let col = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(row)
            .push(options);

        Container::new(col).width(Length::Fill).height(Length::Fill)
    }

    pub fn view_records(&self) -> Container<HomeMessage> {
        let categories = select_list::SelectList::from(&self.categories_list)
            .on_select(HomeMessage::HanldeSelectCategories)
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

        let form = if let Some(selected) = self.categories_list.get(self.selected_index) {
            match &selected.value {
                record::Categories::Login(elem) => {
                    let f = AddRecordForm::from(&elem)
                        .set_read_only(self.read_only)
                        .set_title(t!(&format!(
                            "item_{}",
                            record::Categories::Login(Default::default())
                        )))
                        .on_copy(HomeMessage::Copy);

                    Container::new(f)
                }
                _ => {
                    let ctx = Text::new("not implemented yet");

                    Container::new(ctx)
                }
            }
        } else {
            // TODO: make error hanlder
            let error = Text::new("NOT WORKS");

            Container::new(error)
        };
        let row = Row::new().push(left_search_col).push(vline).push(form);

        Container::new(row)
    }
}
