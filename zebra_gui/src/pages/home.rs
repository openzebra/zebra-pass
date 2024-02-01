//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::{Arc, Mutex};

use iced::{Command, Length, Subscription};
use zebra_lib::{core::core::Core, errors::ZebraErrors};
use zebra_ui::widget::*;

use crate::components::home_nav_bar::{NavBar, NavRoute, LINE_ALFA_CHANNEL};
use crate::gui::GlobalMessage;

use super::Page;

#[derive(Debug)]
pub struct Home {
    core: Arc<Mutex<Core>>,
}

#[derive(Debug, Clone, Copy)]
pub enum HomeMessage {
    RouteGen,
    RouteSettings,
}

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
        let records = &self.core.lock().unwrap().data;
        let content = Container::new(if records.is_empty() {
            self.view_no_records()
        } else {
            self.view_records()
        });

        NavBar::<Self::Message>::new()
            .on_home(HomeMessage::RouteSettings)
            .on_gen(HomeMessage::RouteGen)
            .on_settings(HomeMessage::RouteSettings)
            .view(content)
            .into()
    }
}

impl Home {
    pub fn view_no_records(&self) -> Row<HomeMessage> {
        Row::new()
    }

    pub fn view_records(&self) -> Row<HomeMessage> {
        let vline = zebra_ui::components::line::Line::new()
            .width(Length::Fixed(1.0))
            .height(Length::Fill)
            .alfa(LINE_ALFA_CHANNEL)
            .style(zebra_ui::components::line::LineStyleSheet::Secondary);
        let left_search_col = Column::new().height(Length::Fill).width(200);

        Row::new().push(left_search_col).push(vline)
    }
}
