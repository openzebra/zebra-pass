//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use std::borrow::Cow;

use iced::{
    widget::{component, Column, Component, Container},
    Padding,
};
use iced::{Element, Length, Renderer, Theme};
use rust_i18n::t;
use zebra_ui::components::line::Linear;

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
    on_copy: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_edit_email: Option<Message>,
    on_export_database: Option<Message>,
    on_export_records: Option<Message>,
}

#[derive(Debug, Clone)]
pub enum Event {
    CopyValue(String),
    EditEmail,
    ExportRecords,
    ExportDatabase,
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
            on_copy: None,
            on_edit_email: None,
            on_export_database: None,
            on_export_records: None,
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
            Event::ExportRecords => self.on_export_records.clone(),
            Event::EditEmail => self.on_edit_email.clone(),
            Event::ExportDatabase => self.on_export_database.clone(),
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
            .on_edit(Event::EditEmail)
            .set_value(&self.email);
        let email = Container::new(email);

        let records = SmartFields::new()
            .set_label(t!("amount_of_records"))
            .set_padding(self.item_padding)
            .on_export(Event::ExportRecords)
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

        Container::new(border_col)
            .padding(Padding {
                left: self.item_padding,
                right: self.item_padding,
                top: 0.0,
                bottom: 0.0,
            })
            .width(Length::Fill)
            .style(zebra_ui::styles::container::primary_bordered)
            .into()
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
