//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use crate::components::smart_input::SmartInput;
use crate::{gui::Routers, rust_i18n::t};
use iced::widget::{text_input, Button, Column, Container, Row, Space, Text};
use iced::Element;
use iced::{
    alignment::Horizontal, event, keyboard::key::Named, Command, Event, Length, Subscription,
};
use zebra_lib::{core::Core, errors::ZebraErrors};
use zebra_ui::components::circular;
use zebra_ui::config::PRINT_WIDTH;

use crate::gui::GlobalMessage;

use super::{error::ErrorPage, home::Home, options::Options, Page};

#[derive(Debug)]
pub struct Lock {
    core: Arc<Mutex<Core>>,
    password: String,
    loading: bool,
    loaded: bool,
    err_message: String,
    input_id: text_input::Id,
}

#[derive(Debug, Clone)]
pub enum LockMessage {
    OnPasswordInput(String),
    TabPressed(bool),
    EventOccurred(Event),
    OnOptions,
    OnSubmit,
    OnFinishLoading(Result<(), ZebraErrors>),
}

pub async fn unlock(core: Arc<Mutex<Core>>, passowrd: String) -> Result<(), ZebraErrors> {
    let mut core = core.lock().or(Err(ZebraErrors::SyncStateLock))?;

    core.unlock(&passowrd)?;

    Ok(())
}

impl Page for Lock {
    type Message = LockMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let password = String::new();
        let loading = false;
        let loaded = false;
        let err_message = String::new();
        let input_id = text_input::Id::new("password_id");

        Ok(Self {
            core,
            err_message,
            input_id,
            loaded,
            loading,
            password,
        })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch([
            event::listen().map(LockMessage::EventOccurred),
            iced::keyboard::on_key_press(|key_code, modifiers| match (key_code, modifiers) {
                (iced::keyboard::Key::Named(Named::Tab), _) => {
                    Some(LockMessage::TabPressed(modifiers.shift()))
                }
                _ => None,
            }),
        ])
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<GlobalMessage> {
        match message {
            LockMessage::OnSubmit => {
                if self.password.is_empty() {
                    return text_input::focus::<GlobalMessage>(self.input_id.clone());
                }

                self.loading = true;

                Command::perform(unlock(Arc::clone(&self.core), self.password.clone()), |r| {
                    GlobalMessage::LockMessage(LockMessage::OnFinishLoading(r))
                })
            }
            LockMessage::OnOptions => match Options::new(Arc::clone(&self.core)) {
                Ok(options) => {
                    let route = Routers::Options(options);

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
                Err(e) => {
                    let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));

                    Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                }
            },
            LockMessage::OnPasswordInput(v) => {
                self.err_message = String::new();
                self.password = v;
                Command::none()
            }
            LockMessage::TabPressed(shift) => {
                if shift {
                    iced::widget::focus_previous()
                } else {
                    iced::widget::focus_next()
                }
            }
            LockMessage::EventOccurred(_) => {
                if !self.loaded {
                    self.loaded = true;
                    text_input::focus::<GlobalMessage>(self.input_id.clone())
                } else {
                    Command::none()
                }
            }
            LockMessage::OnFinishLoading(result) => match result {
                Ok(_) => match Home::new(Arc::clone(&self.core)) {
                    Ok(home) => {
                        let route = Routers::Home(home);

                        Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                    }
                    Err(e) => {
                        let route = Routers::ErrorPage(ErrorPage::from(e.to_string()));

                        Command::perform(std::future::ready(1), |_| GlobalMessage::Route(route))
                    }
                },
                Err(e) => {
                    self.loading = false;
                    self.err_message = e.to_string();

                    text_input::focus::<GlobalMessage>(self.input_id.clone())
                }
            },
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let zebra_print = zebra_ui::image::zebra_print_view();
        let error_message = Text::new(&self.err_message)
            .size(14)
            .style(zebra_ui::styles::text::danger)
            .horizontal_alignment(Horizontal::Center);
        let mut passowrd_input = SmartInput::new()
            .set_value(&self.password)
            .padding(10)
            .set_danger(!self.err_message.is_empty())
            .set_font_size(14)
            .set_secure(true)
            .set_placeholder(t!("placeholder_password"));

        if !self.loading {
            passowrd_input = passowrd_input
                .on_input(LockMessage::OnPasswordInput)
                .on_submit(LockMessage::OnSubmit);
        }

        let passowrd_input = Container::new(passowrd_input).width(250);
        let submit_btn = Button::new(
            Text::new(t!("unlock_btn"))
                .horizontal_alignment(Horizontal::Center)
                .width(Length::Fill)
                .size(16),
        )
        .padding(8)
        .width(250)
        .height(38)
        .style(zebra_ui::styles::button::outline_primary)
        .on_press(LockMessage::OnSubmit);
        let loading_btn = Button::new(
            Column::new()
                .push(circular::Circular::new().size(20.0))
                .width(Length::Fill)
                .align_items(iced::Alignment::Center),
        )
        .padding(8)
        .height(38)
        .style(zebra_ui::styles::button::outline_primary)
        .width(250);
        let options_btn = Button::new(Text::new(t!("restore_or_create")).size(14))
            .style(zebra_ui::styles::button::ref_primary)
            .on_press_maybe(if self.loading {
                None
            } else {
                Some(LockMessage::OnOptions)
            });
        let options_col = Column::new()
            .width(250)
            .push(options_btn)
            .align_items(iced::Alignment::Start);
        let lock_icon = zebra_ui::image::lock_icon().width(100).height(100);
        let print_col = Column::new()
            .width(PRINT_WIDTH)
            .height(Length::Fill)
            .push(zebra_print);
        let payload_col = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center)
            .padding(50)
            .push(lock_icon)
            .push(Space::new(0.0, 16.0))
            .push(error_message)
            .push(Space::new(0.0, 5.0))
            .push(passowrd_input)
            .push(Space::new(0.0, 5.0))
            .push(match self.loading {
                false => submit_btn,
                true => loading_btn,
            })
            .push(options_col);
        let row = Row::new()
            .width(Length::Fill)
            .push(print_col)
            .push(payload_col);

        Container::new(row)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
