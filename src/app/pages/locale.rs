//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use crate::app::message::{GlobalMessages, RouteMessages};
use crate::rust_i18n::t;
use crate::settings::language::Language;
use crate::{_rust_i18n_translate, app::style::button::ZButton};
use iced::widget::button::StyleSheet;
use iced::Alignment;
use iced::{
    alignment::Horizontal,
    widget::{button, container, radio, scrollable, text, Column, Container, Row, Space},
    Length, Point, Rectangle, Size,
};

use crate::{app::components::zebra_print::zebra_print_view, core::core::Core};

pub struct LocalePage<'a> {
    core: &'a Core,
}

#[derive(Debug, Clone, Copy)]
pub enum LocaleMessage {}

impl<'a> LocalePage<'a> {
    pub fn from(core: &'a Core) -> Self {
        Self { core }
    }

    pub fn view<'b>(&self) -> Container<'b, GlobalMessages> {
        let zebra_print = zebra_print_view();

        let title = text(t!("welcome"))
            .size(60)
            .horizontal_alignment(Horizontal::Center);

        let btn = button(
            text("Next")
                .size(20)
                .horizontal_alignment(Horizontal::Center),
        )
        .style(ZButton::outline_primary().into())
        .on_press(GlobalMessages::Route(RouteMessages::Back))
        .width(120);

        let scrollable_col = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(2)
            .push(
                button(
                    text(Language::Russian.to_string())
                        .size(15)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                )
                .style(ZButton::list_item().into())
                .width(Length::Fill),
            )
            .push(
                button(
                    text(Language::Danish.to_string())
                        .size(15)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                )
                .style(ZButton::list_item().into())
                .width(Length::Fill),
            )
            .push(
                button(
                    text(Language::French.to_string())
                        .size(15)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                )
                .style(ZButton::list_item().into())
                .width(Length::Fill),
            )
            .push(
                button(
                    text(Language::German.to_string())
                        .size(15)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                )
                .style(ZButton::list_item().into())
                .width(Length::Fill),
            )
            .push(
                button(
                    text(Language::English.to_string())
                        .size(15)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                )
                .style(ZButton::list_item().into())
                .width(Length::Fill),
            )
            .push(
                button(
                    text(Language::Italian.to_string())
                        .size(15)
                        .horizontal_alignment(iced::alignment::Horizontal::Center),
                )
                .style(ZButton::list_item().into())
                .width(Length::Fill),
            );

        let scroll = scrollable(scrollable_col).height(Length::Fill);
        let print_col = Column::new()
            .width(200)
            .height(Length::Fill)
            .push(zebra_print);
        let payload_col = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center)
            .padding(50)
            .push(title)
            .push(scroll)
            .push(btn);

        let row = Row::new()
            .width(Length::Fill)
            .push(print_col)
            .push(payload_col);

        Container::new(row).height(Length::Fill).width(Length::Fill)
    }
}
