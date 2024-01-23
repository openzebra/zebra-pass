//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::{
    str::Lines,
    sync::{Arc, Mutex},
};

use iced::{Command, Length, Subscription};
use zebra_lib::{core::core::Core, errors::ZebraErrors};
use zebra_ui::widget::*;

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
        let header = self.view_header();
        let content_col = self.view_content();
        let left_bar_col = Column::new().height(Length::Fill).width(60);
        let main_row = Row::new().push(left_bar_col).push(content_col);
        let main_col = Column::new().push(header).push(main_row);

        Container::new(main_col)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}

impl Home {
    pub fn view_header(&self) -> Container<HomeMessage> {
        let header_col = Column::new();
        Container::new(header_col).width(Length::Fill).height(60)
    }

    pub fn view_content(&self) -> Container<HomeMessage> {
        let content_col = Column::new();
        Container::new(content_col)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(zebra_ui::style::container::Container::WeekBorder)
    }
}
