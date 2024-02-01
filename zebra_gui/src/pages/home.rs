//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use iced::{Command, Length, Subscription};
use zebra_lib::{core::core::Core, errors::ZebraErrors};
use zebra_ui::widget::*;

use crate::components::home_nav_bar::{NavBar, NavRoute};
use crate::gui::GlobalMessage;

use super::Page;

#[derive(Debug)]
pub struct Home {
    core: Arc<Mutex<Core>>,
}

#[derive(Debug, Clone, Copy)]
pub enum HomeMessage {}

impl Page for Home {
    type Message = HomeMessage;

    fn new(core: Arc<Mutex<Core>>) -> Result<Self, ZebraErrors> {
        Ok(Self { core })
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::none()
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<GlobalMessage> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let main_row = Row::new();
        let content = Container::new(main_row);

        NavBar::<Self::Message>::new().view(content).into()
    }
}
