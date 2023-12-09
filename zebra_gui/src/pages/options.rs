//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};
use zebra_lib::{core::core::Core, errors::ZebraErrors};

use crate::{
    gui::{GlobalMessage, Routers},
    rust_i18n::t,
};

use super::Page;
use iced::{alignment::Horizontal, Command, Length, Subscription};
use zebra_ui::widget::*;

#[derive(Debug)]
pub struct Options {
    core: Arc<Mutex<Core>>,
}

#[derive(Debug)]
pub enum OptionsMessage {
    Back,
    Restore,
    Create,
}

impl Page for Options {
    type Message = OptionsMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        Ok(Self { core })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> Command<GlobalMessage> {
        match message {
            OptionsMessage::Back => Command::none(),
            OptionsMessage::Create => Command::none(),
            OptionsMessage::Restore => Command::none(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let zebra_print = zebra_ui::image::zebra_print_view();
        let print_col = Column::new()
            .width(220)
            .height(Length::Fill)
            .push(zebra_print);
        let title = Text::new(t!("zebra_name"))
            .size(34)
            .horizontal_alignment(Horizontal::Center);
        let col = Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(15)
            .align_items(iced::Alignment::Center)
            .push(title);
        let row = Row::new().width(Length::Fill).push(print_col).push(col);

        Container::new(row)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
