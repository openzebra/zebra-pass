//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::widget::{component, text_input, Button, Column, Component, Container, Row, Space, Text};
use iced::{Element, Renderer, Theme};

pub struct SmartInput<'a, Message>
where
    Message: Clone,
{
    showed_secure_flag: bool,
    on_reload: Option<Message>,
    on_copy: Option<Message>,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_submit: Option<Message>,
    padding: u16,
    secured: bool,
    placeholder: String,
    value: String,
    label: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Event {
    Copy,
    Reload,
    ShowHideSecure,
    HandleInput(String),
}

impl<'a, Message: Clone> SmartInput<'a, Message>
where
    Message: Clone,
{
    pub fn new() -> Self {
        let padding = 0;
        let secured = false;
        let placeholder = String::new();
        let value = String::new();
        let label = None;

        Self {
            padding,
            label,
            secured,
            value,
            placeholder,
            showed_secure_flag: false,
            on_reload: None,
            on_copy: None,
            on_input: None,
            on_submit: None,
        }
    }

    pub fn set_secure(mut self, flag: bool) -> Self {
        self.secured = flag;

        self
    }

    pub fn set_placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = placeholder;

        self
    }

    pub fn set_reload(mut self, reload_msg: Message) -> Self {
        self.on_reload = Some(reload_msg);

        self
    }

    pub fn set_copy(mut self, copy_msg: Message) -> Self {
        self.on_copy = Some(copy_msg);

        self
    }

    pub fn on_input<F>(mut self, callback: F) -> Self
    where
        F: 'a + Fn(String) -> Message,
    {
        self.on_input = Some(Box::new(callback));

        self
    }

    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    pub fn padding(mut self, amount: u16) -> Self {
        self.padding = amount;

        self
    }
}

impl<'a, Message> Component<Message, Theme, Renderer> for SmartInput<'a, Message>
where
    Message: Clone,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::Copy => self.on_copy.clone(),
            Event::Reload => self.on_reload.clone(),
            Event::ShowHideSecure => {
                self.showed_secure_flag = !self.showed_secure_flag;

                None
            }
            Event::HandleInput(v) => {
                self.value = v;

                None
            }
        }
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer> {
        let mut input = text_input(&self.placeholder, &self.value)
            .size(14)
            .padding(self.padding)
            .secure(self.secured && !self.showed_secure_flag)
            .style(zebra_ui::styles::input::transparent_primary);

        if self.on_input.is_some() {
            input = input.on_input(Event::HandleInput);
        }

        let col = if let Some(label) = &self.label {
            let label = Text::new(label.clone()).size(12);

            Column::new().push(label).push(input)
        } else {
            Column::new().push(input)
        };

        let mut row = Row::new().align_items(iced::Alignment::Center).push(col);

        if self.secured {
            let icon = if self.showed_secure_flag {
                zebra_ui::image::open_eye_icon()
            } else {
                zebra_ui::image::close_eye_icon()
            }
            .style(zebra_ui::styles::svg::primary_hover)
            .height(25)
            .width(25);
            let eye_btn = Button::new(icon)
                .padding(0)
                .style(zebra_ui::styles::button::transparent)
                .on_press_maybe(if self.on_input.is_some() {
                    Some(Event::ShowHideSecure)
                } else {
                    None
                });
            row = row.push(eye_btn);
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
            .on_press_maybe(if self.on_input.is_some() {
                Some(Event::Copy)
            } else {
                None
            });
            row = row.push(copy_btn);
        }

        if self.on_reload.is_some() {
            let reload_btn = Button::new(
                zebra_ui::image::reload_icon()
                    .style(zebra_ui::styles::svg::primary_hover)
                    .height(30)
                    .width(30),
            )
            .padding(0)
            .style(zebra_ui::styles::button::transparent)
            .on_press_maybe(if self.on_input.is_some() {
                Some(Event::Reload)
            } else {
                None
            });
            row = row.push(reload_btn);
        }

        row = row.push(Space::new(5, 0));

        Container::new(row)
            .style(zebra_ui::styles::container::primary_bordered)
            .into()
    }
}

impl<'a, Message> From<SmartInput<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(form: SmartInput<'a, Message>) -> Self {
        component(form)
    }
}
