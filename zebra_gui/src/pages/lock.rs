//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use crate::rust_i18n::t;
use iced::{
    alignment::Horizontal,
    event,
    widget::{text_input, Space},
    Command, Event, Length, Subscription,
};
use zebra_lib::{core::core::Core, errors::ZebraErrors};
use zebra_ui::widget::*;

use crate::gui::GlobalMessage;

use super::Page;

#[derive(Debug)]
pub struct Lock {
    core: Arc<Mutex<Core>>,
    password: String,
    show: bool,
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
    OnSubmit,
}

impl Page for Lock {
    type Message = LockMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let password = String::new();
        let show = false;
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
            show,
        })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch([
            event::listen().map(LockMessage::EventOccurred),
            iced::keyboard::on_key_press(|key_code, modifiers| match (key_code, modifiers) {
                (iced::keyboard::KeyCode::Tab, _) => {
                    Some(LockMessage::TabPressed(modifiers.shift()))
                }
                _ => None,
            }),
        ])
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<GlobalMessage> {
        match message {
            LockMessage::OnSubmit => {
                dbg!("submited");
                Command::none()
            }
            LockMessage::OnPasswordInput(v) => {
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
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let zebra_print = zebra_ui::image::zebra_print_view();
        let title = Text::new(t!("welcome"))
            .size(34)
            .horizontal_alignment(Horizontal::Center);
        let error_message = Text::new(&self.err_message)
            .size(14)
            .style(zebra_ui::style::text::Text::Dabger)
            .horizontal_alignment(Horizontal::Center);
        let mut passowrd = text_input(&t!("placeholder_password"), &self.password)
            .size(16)
            .padding(8)
            .width(250)
            .id(self.input_id.clone())
            .style(zebra_ui::style::text_input::TextInput::Primary);

        if !self.loading {
            passowrd = passowrd
                .on_input(LockMessage::OnPasswordInput)
                .on_submit(LockMessage::OnSubmit);
        }
        if !self.show {
            passowrd = passowrd.password();
        }

        let submit_btn = Button::new(
            Text::new(t!("create_btn"))
                .horizontal_alignment(Horizontal::Center)
                .width(Length::Fill)
                .size(16),
        )
        .padding(8)
        .width(250)
        .on_press(LockMessage::OnSubmit)
        .style(zebra_ui::style::button::Button::OutlinePrimary);
        let print_col = Column::new()
            .width(220)
            .height(Length::Fill)
            .push(zebra_print);
        let payload_col = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center)
            .padding(50)
            .push(title)
            .push(Space::new(0.0, 16.0))
            .push(error_message)
            .push(Space::new(0.0, 5.0))
            .push(passowrd)
            .push(Space::new(0.0, 5.0))
            .push(submit_btn);
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
