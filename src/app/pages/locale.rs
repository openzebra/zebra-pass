//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::{
    widget::{Container, Text},
    Element, Length,
};

use crate::core::core::Core;

pub struct LocalePage<'a> {
    core: &'a Core,
}

#[derive(Debug)]
pub enum Message {}

impl<'a> LocalePage<'a> {
    pub fn new(core: &Core) -> Self {
        Self { core }
    }

    pub fn view(&self) -> Element<Message> {
        Container::new(Text::new("Hello from Page 2"))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
