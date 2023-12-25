//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use crate::rust_i18n::t;
use iced::{alignment::Horizontal, widget::text_input, Command, Length, Subscription};
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
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<GlobalMessage> {
        match message {
            LockMessage::OnPasswordInput(v) => {
                self.password = v;
                Command::none()
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
            .width(250)
            .password()
            .style(zebra_ui::style::text_input::TextInput::Primary);

        if !self.loading {
            passowrd = passowrd.on_input(LockMessage::OnPasswordInput);
        }

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
            .push(passowrd);

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
