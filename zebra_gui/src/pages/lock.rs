//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use crate::rust_i18n::t;
use iced::{
    alignment::Horizontal,
    widget::{text_input, Space},
    Command, Length, Subscription,
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
}

#[derive(Debug, Clone)]
pub enum LockMessage {
    OnPasswordInput(String),
    TabPressed(bool),
    OnSubmit,
}

impl Page for Lock {
    type Message = LockMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        let password = String::new();
        let show = false;
        let loading = false;

        Ok(Self {
            core,
            loading,
            password,
            show,
        })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::keyboard::on_key_press(|key_code, modifiers| match (key_code, modifiers) {
            (iced::keyboard::KeyCode::Tab, _) => Some(LockMessage::TabPressed(modifiers.shift())),
            _ => None,
        })
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<GlobalMessage> {
        match message {
            LockMessage::OnPasswordInput(v) => {
                self.password = v;
                Command::none()
            }
            LockMessage::OnSubmit => Command::none(),
            LockMessage::TabPressed(shift) => {
                if shift {
                    iced::widget::focus_previous()
                } else {
                    iced::widget::focus_next()
                }
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let zebra_print = zebra_ui::image::zebra_print_view();
        let title = Text::new(t!("welcome"))
            .size(34)
            .horizontal_alignment(Horizontal::Center);
        let mut passowrd = text_input(&t!("placeholder_password"), &self.password)
            .size(16)
            .padding(8)
            .width(250)
            .password()
            .style(zebra_ui::style::text_input::TextInput::Primary);

        if !self.loading {
            passowrd = passowrd.on_input(LockMessage::OnPasswordInput);
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
