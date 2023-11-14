//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use std::{cell::RefCell, rc::Rc};

use iced::{
    widget::{Container, Text},
    Element, Length,
};

use crate::{app::app::RouteMessages, core::core::Core};

pub struct LocalePage<'a> {
    core: &'a Core,
}

#[derive(Debug)]
pub enum LocaleMessage {}

impl<'a> LocalePage<'a> {
    pub fn from(core: &'a Core) -> Self {
        Self { core }
    }

    pub fn view<'b>(&self) -> Container<'b, RouteMessages> {
        Container::new(Text::new("Hello from Page 2").size(20))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
    }
}
