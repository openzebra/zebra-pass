//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use std::borrow::Cow;

use iced::{
    widget::{component, text_input, Button, Column, Component, Container, Row, Space, Text},
    Padding,
};
use iced::{Element, Length, Renderer, Theme};
use rust_i18n::t;
use zebra_ui::components::line::Linear;

use super::modal::Modal;
use crate::components::{home_nav_bar::LINE_ALFA_CHANNEL, smart_field::SmartFields};

pub struct ProfileViewForm<'a, Message>
where
    Message: Clone,
{
    email: Cow<'a, str>,
    address: Cow<'a, str>,
    data_dir_path: Cow<'a, str>,
    storage_version: String,
    data_size: String,
    records_len: String,
    main_padding: f32,
    item_padding: f32,
    edit_email_modal: bool,
    export_records_modal: bool,
    on_copy: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_edit_email: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_export_database: Option<Message>,
    on_export_records: Option<Message>,
}

#[derive(Debug, Clone)]
pub enum Event {
    CopyValue(String),
    EditEmailModal,
    ExportRecordsModal,
    ExportRecords,
    EditEmail,
    ExportDatabase,
    InputEmail(String),
}

impl<'a, Message> Default for ProfileViewForm<'a, Message>
where
    Message: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message: Clone> ProfileViewForm<'a, Message>
where
    Message: Clone,
{
    pub fn new() -> Self {
        Self {
            email: Cow::default(),
            address: Cow::default(),
            data_dir_path: Cow::default(),
            storage_version: String::new(),
            data_size: String::new(),
            records_len: String::new(),
            item_padding: Default::default(),
            main_padding: Default::default(),
            edit_email_modal: false,
            export_records_modal: false,
            on_copy: None,
            on_edit_email: None,
            on_export_records: None,
            on_export_database: None,
        }
    }

    pub fn set_item_padding(mut self, value: f32) -> Self {
        self.item_padding = value;
        self
    }

    pub fn set_main_padding(mut self, value: f32) -> Self {
        self.main_padding = value;
        self
    }

    pub fn set_records_len(mut self, value: usize) -> Self {
        self.records_len = value.to_string();
        self
    }

    pub fn set_data_size(mut self, value: u64) -> Self {
        self.data_size = format!("{} bytes", value);
        self
    }

    pub fn set_storage_version(mut self, value: u16) -> Self {
        self.storage_version = format!("V{}", value);
        self
    }

    pub fn set_email(mut self, value: Cow<'a, str>) -> Self {
        self.email = value;
        self
    }

    pub fn set_address(mut self, value: Cow<'a, str>) -> Self {
        self.address = value;
        self
    }

    pub fn set_data_dir_path(mut self, value: Cow<'a, str>) -> Self {
        self.data_dir_path = value;
        self
    }

    pub fn on_edit_email<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(String) -> Message,
    {
        self.on_edit_email = Some(Box::new(callback));

        self
    }

    pub fn on_export_records(mut self, msg: Message) -> Self {
        self.on_export_records = Some(msg);
        self
    }

    pub fn on_export_database(mut self, msg: Message) -> Self {
        self.on_export_database = Some(msg);
        self
    }

    pub fn on_copy<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(String) -> Message,
    {
        self.on_copy = Some(Box::new(callback));

        self
    }

    pub fn view_hline(&self) -> Linear<Theme> {
        Linear::new()
            .height(Length::Fixed(0.5))
            .width(Length::Fill)
            .style(zebra_ui::styles::line::line_secondary)
            .alfa(LINE_ALFA_CHANNEL)
    }

    pub fn view_edit_email_modal(&self) -> Container<'a, Event, Theme, Renderer> {
        let close_btn = Button::new(
            zebra_ui::image::close_icon()
                .style(zebra_ui::styles::svg::primary_hover)
                .height(30)
                .width(30),
        )
        .padding(0)
        .style(zebra_ui::styles::button::transparent)
        .on_press(Event::EditEmailModal);
        let close_btn = Column::new()
            .push(close_btn)
            .width(Length::Fill)
            .align_items(iced::Alignment::End);
        let row_header = Row::new().padding(8).push(close_btn).width(Length::Fill);
        let description = Text::new(t!("edit_email_description"))
            .size(14)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .style(zebra_ui::styles::text::warn);
        let email_input = text_input(&t!("placeholder_email"), &self.email)
            .size(14)
            .width(250)
            .on_submit(Event::EditEmail)
            .on_input(Event::InputEmail)
            .padding(8)
            .style(zebra_ui::styles::input::primary);

        let save_btn = Button::new(
            Text::new(t!("save_email_btn"))
                .size(self.item_padding * 2.0)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .style(zebra_ui::styles::button::outline_primary)
        .padding(self.item_padding)
        .on_press(Event::EditEmail);

        let main_modal_col = Column::new()
            .push(row_header)
            .push(description)
            .push(Space::new(0, 8))
            .push(email_input)
            .push(Space::new(0, 8))
            .push(save_btn)
            .push(Space::new(0, self.item_padding))
            .padding(self.item_padding)
            .align_items(iced::Alignment::Center);

        Container::new(main_modal_col)
            .width(400)
            .style(zebra_ui::styles::container::primary_bordered_modal)
    }

    pub fn view_export_records_modal(&self) -> Container<'a, Event, Theme, Renderer> {
        let close_btn = Button::new(
            zebra_ui::image::close_icon()
                .style(zebra_ui::styles::svg::primary_hover)
                .height(30)
                .width(30),
        )
        .padding(0)
        .style(zebra_ui::styles::button::transparent)
        .on_press(Event::ExportRecordsModal);
        let close_btn = Column::new()
            .push(close_btn)
            .width(Length::Fill)
            .align_items(iced::Alignment::End);
        let row_header = Row::new().padding(8).push(close_btn).width(Length::Fill);
        let description = Text::new(t!("export_records_description"))
            .size(14)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .style(zebra_ui::styles::text::warn);

        let export_btn = Button::new(
            Text::new(t!("export_btn"))
                .size(self.item_padding * 2.0)
                .horizontal_alignment(iced::alignment::Horizontal::Center),
        )
        .style(zebra_ui::styles::button::outline_primary)
        .padding(self.item_padding)
        .on_press(Event::ExportRecords);

        let main_modal_col = Column::new()
            .push(row_header)
            .push(Space::new(0, 8))
            .push(description)
            .push(Space::new(0, 8))
            .push(export_btn)
            .push(Space::new(0, self.item_padding))
            .padding(self.item_padding)
            .align_items(iced::Alignment::Center);

        Container::new(main_modal_col)
            .width(400)
            .style(zebra_ui::styles::container::primary_bordered_modal)
    }
}

impl<'a, Message> Component<Message, Theme, Renderer> for ProfileViewForm<'a, Message>
where
    Message: Clone,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::CopyValue(value) => self.on_copy.as_ref().map(|e| e(value)),
            Event::ExportRecords => {
                self.export_records_modal = !self.export_records_modal;
                self.on_export_records.clone()
            }
            Event::EditEmail => self
                .on_edit_email
                .as_ref()
                .map(|cb| cb(self.email.to_string())),
            Event::ExportDatabase => self.on_export_database.clone(),
            Event::InputEmail(value) => {
                self.email = value.into();

                None
            }
            Event::ExportRecordsModal => {
                self.export_records_modal = !self.export_records_modal;

                None
            }
            Event::EditEmailModal => {
                self.edit_email_modal = !self.edit_email_modal;

                None
            }
        }
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer> {
        let address = SmartFields::new()
            .set_label(t!("address"))
            .set_padding(self.item_padding)
            .set_truncate(true)
            .on_copy(Event::CopyValue)
            .set_value(&self.address);
        let address = Container::new(address);

        let email = SmartFields::new()
            .set_label(t!("email"))
            .set_padding(self.item_padding)
            .on_copy(Event::CopyValue)
            .on_edit(Event::EditEmailModal)
            .set_value(&self.email);
        let email = Container::new(email);

        let records = SmartFields::new()
            .set_label(t!("amount_of_records"))
            .set_padding(self.item_padding)
            .on_export(Event::ExportRecordsModal)
            .set_value(&self.records_len);
        let records = Container::new(records);

        let data_dir = SmartFields::new()
            .set_label(t!("database_path"))
            .set_padding(self.item_padding)
            .on_copy(Event::CopyValue)
            .on_export(Event::ExportDatabase)
            .set_value(&self.data_dir_path);
        let data_dir = Container::new(data_dir);

        let data_size = SmartFields::new()
            .set_label(t!("database_size"))
            .set_padding(self.item_padding)
            .set_value(&self.data_size);
        let data_size = Container::new(data_size);

        let storage_version = SmartFields::new()
            .set_label(t!("storage_version"))
            .set_padding(self.item_padding)
            .set_value(&self.storage_version);
        let storage_version = Container::new(storage_version);

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
        let main_content = Container::new(border_col)
            .padding(Padding {
                left: self.item_padding,
                right: self.item_padding,
                top: 0.0,
                bottom: 0.0,
            })
            .width(Length::Fill)
            .style(zebra_ui::styles::container::primary_bordered);

        if self.export_records_modal {
            Modal::new(main_content, self.view_export_records_modal())
                .on_blur(Event::ExportRecordsModal)
                .into()
        } else if self.edit_email_modal {
            Modal::new(main_content, self.view_edit_email_modal())
                .on_blur(Event::EditEmailModal)
                .into()
        } else {
            Container::new(main_content).into()
        }
    }
}

impl<'a, Message> From<ProfileViewForm<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(form: ProfileViewForm<'a, Message>) -> Self {
        component(form)
    }
}
