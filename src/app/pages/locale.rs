//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::{
    alignment::Horizontal,
    widget::{button, container, radio, scrollable, text, Column, Container, Row},
    Length,
};

use crate::{
    app::{app::RouteMessages, components::zebra_print::zebra_print_view},
    core::core::Core,
};

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
        let zebra_print = zebra_print_view();
        let title = text("Choose language")
            .size(60)
            .horizontal_alignment(Horizontal::Center);

        let print_col = Column::new()
            .width(200)
            .height(Length::Fill)
            .push(zebra_print);
        let payload_col = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center)
            .push(title);
        let row = Row::new()
            .width(Length::Fill)
            .push(print_col)
            .push(payload_col);

        Container::new(row).height(Length::Fill).width(Length::Fill)
    }
}
