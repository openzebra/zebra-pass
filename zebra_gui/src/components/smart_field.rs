//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use std::borrow::Cow;

use iced::widget::{component, Button, Column, Component, Container, Row, Text};
use iced::{Element, Length, Padding, Renderer, Theme};
use zebra_lib::utils::truncate_string;

pub struct SmartFields<'a, Message>
where
    Message: Clone,
{
    on_copy: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_edit: Option<Message>,
    container_padding: Padding,
    label_size: u16,
    value_size: u16,
    label: Cow<'a, str>,
    value: Cow<'a, str>,
    use_truncate: bool,
}

#[derive(Debug, Clone)]
pub enum Event {
    Copy,
    Edit,
}

impl<'a, Message> Default for SmartFields<'a, Message>
where
    Message: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> SmartFields<'a, Message>
where
    Message: Clone,
{
    pub fn new() -> Self {
        Self {
            container_padding: Padding::ZERO,
            on_copy: None,
            on_edit: None,
            label_size: 16,
            value_size: 14,
            label: Cow::default(),
            value: Cow::default(),
            use_truncate: false,
        }
    }

    pub fn set_truncate(mut self, value: bool) -> Self {
        self.use_truncate = value;
        self
    }

    pub fn set_label(mut self, label: Cow<'a, str>) -> Self {
        self.label = label;
        self
    }

    pub fn set_value(mut self, value: Cow<'a, str>) -> Self {
        self.value = value;
        self
    }

    pub fn set_label_size(mut self, size: u16) -> Self {
        self.label_size = size;
        self
    }

    pub fn set_value_size(mut self, size: u16) -> Self {
        self.value_size = size;
        self
    }

    pub fn set_padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.container_padding = padding.into();
        self
    }

    pub fn on_edit(mut self, msg: Message) -> Self {
        self.on_edit = Some(msg);
        self
    }

    pub fn on_copy<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(String) -> Message,
    {
        self.on_copy = Some(Box::new(callback));

        self
    }
}

impl<'a, Message> Component<Message, Theme, Renderer> for SmartFields<'a, Message>
where
    Message: Clone,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::Copy => {
                if let Some(cb) = &self.on_copy {
                    Some(cb(self.value.to_string()))
                } else {
                    None
                }
            }
            Event::Edit => self.on_edit.clone(),
        }
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer> {
        let title = Text::new(self.label.as_ref()).size(self.label_size);
        let value = Text::new(if self.use_truncate {
            truncate_string(self.value.as_ref(), 20)
        } else {
            self.value.clone()
        })
        .size(self.value_size)
        .style(zebra_ui::styles::text::muted);
        let col = Column::new().width(Length::Fill).push(title).push(value);
        let mut row = Row::new()
            .spacing(5)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(col);

        if self.on_edit.is_some() {
            let edit_btn = Button::new(
                zebra_ui::image::edit_icon()
                    .style(zebra_ui::styles::svg::primary_hover)
                    .height(25)
                    .width(25),
            )
            .padding(0)
            .style(zebra_ui::styles::button::transparent)
            .on_press(Event::Edit);

            row = row.push(edit_btn);
        }

        if self.on_copy.is_some() {
            let copy_btn = Button::new(
                zebra_ui::image::copy_icon()
                    .style(zebra_ui::styles::svg::primary_hover)
                    .height(25)
                    .width(25),
            )
            .padding(0)
            .style(zebra_ui::styles::button::transparent)
            .on_press(Event::Copy);

            row = row.push(copy_btn);
        }

        Container::new(row)
            .padding(self.container_padding)
            .width(Length::Fill)
            .into()
    }
}

impl<'a, Message> From<SmartFields<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(form: SmartFields<'a, Message>) -> Self {
        component(form)
    }
}
