//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};
use zebra_lib::{core::core::Core, errors::ZebraErrors};

use crate::gui::GlobalMessage;

use super::Page;
use iced::{widget::Column, Command, Subscription};
use zebra_ui::widget::*;

#[derive(Debug)]
pub struct Options {}

#[derive(Debug)]
pub enum OptionsMessage {}

impl Page for Options {
    type Message = OptionsMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        Ok(Self {})
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, message: Self::Message) -> Command<GlobalMessage> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        Column::new().into()
    }
}
