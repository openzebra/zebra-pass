//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::widget::{component, text_input, Button, Column, Component, Container, Row, Space, Text};
use iced::{Element, Renderer, Theme};
use std::borrow::Cow;

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
    placeholder: Cow<'a, str>,
    value: &'a str,
    label: Option<&'a str>,
    font_size: u16,
    label_size: u16,
    danger: bool,
}

#[derive(Debug, Clone)]
pub enum Event {
    Copy,
    Reload,
    ShowHideSecure,
    HandleSubmit,
    HandleInput(String),
}

impl<'a, Message> Default for SmartInput<'a, Message>
where
    Message: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> SmartInput<'a, Message>
where
    Message: Clone,
{
    pub fn new() -> Self {
        let padding = 0;
        let secured = false;
        let placeholder = Cow::default();
        let value = "";
        let label = None;
        let font_size = 14;
        let danger = false;
        let label_size = 12;

        Self {
            padding,
            danger,
            font_size,
            label,
            label_size,
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

    pub fn set_font_size(mut self, amount: u16) -> Self {
        self.font_size = amount;

        self
    }

    pub fn set_placeholder(mut self, placeholder: Cow<'a, str>) -> Self {
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

    pub fn set_value(mut self, value: &'a str) -> Self {
        self.value = value;

        self
    }

    pub fn set_label_size(mut self, value: u16) -> Self {
        self.label_size = value;

        self
    }

    pub fn set_label(mut self, value: &'a str) -> Self {
        self.label = Some(value);

        self
    }

    pub fn set_danger(mut self, is_danger: bool) -> Self {
        self.danger = is_danger;

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
            Event::HandleInput(v) => self.on_input.as_ref().map(|cb| cb(v)),
            Event::HandleSubmit => self.on_submit.clone(),
        }
    }

    fn view(
        &self,
        _state: &Self::State,
    ) -> iced::advanced::graphics::core::Element<'_, Self::Event, Theme, Renderer> {
        let mut input = text_input(&self.placeholder, self.value)
            .size(self.font_size)
            .padding(self.padding)
            .secure(self.secured && !self.showed_secure_flag)
            .style(if self.danger {
                zebra_ui::styles::input::transparent_danger
            } else {
                zebra_ui::styles::input::transparent_primary
            });

        if self.on_input.is_some() {
            input = input.on_input(Event::HandleInput);
        }
        if self.on_submit.is_some() {
            input = input.on_submit(Event::HandleSubmit);
        }

        let col = if let Some(label) = &self.label {
            let label = Text::new(label.to_string())
                .size(self.label_size)
                .line_height(0.1)
                .style(zebra_ui::styles::text::muted);
            let label_row = Row::new().push(Space::new(5, 0)).push(label);

            Column::new()
                .push(Space::new(0, 15))
                .push(label_row)
                .push(input)
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
            .style(if self.on_input.is_some() {
                zebra_ui::styles::svg::primary_hover
            } else {
                zebra_ui::styles::svg::primary_disabled
            })
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
            row = row.spacing(4).push(eye_btn);
        }

        if self.on_copy.is_some() {
            let copy_btn = Button::new(
                zebra_ui::image::copy_icon()
                    .style(if self.on_input.is_some() {
                        zebra_ui::styles::svg::primary_hover
                    } else {
                        zebra_ui::styles::svg::primary_disabled
                    })
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
                    .style(if self.on_input.is_some() {
                        zebra_ui::styles::svg::primary_hover
                    } else {
                        zebra_ui::styles::svg::primary_disabled
                    })
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
            .style(if self.danger {
                if self.on_input.is_some() {
                    zebra_ui::styles::container::danger_bordered_hover
                } else {
                    zebra_ui::styles::container::danger_bordered_disabled
                }
            } else if self.on_input.is_some() {
                zebra_ui::styles::container::primary_bordered_hover
            } else {
                zebra_ui::styles::container::primary_bordered_disabled
            })
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
