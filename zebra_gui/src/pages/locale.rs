//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use crate::rust_i18n::t;
use iced::{alignment::Horizontal, Alignment, Command, Length, Subscription};
use zebra_ui::widget::*;

#[derive(Debug)]
pub struct Locale {
    available_locales: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum LocaleMessage {
    Chose,
    Next,
}

impl Locale {
    pub fn new() -> Self {
        let available_locales = rust_i18n::available_locales!()
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<String>>();

        Self { available_locales }
    }

    pub fn subscription(&self) -> Subscription<LocaleMessage> {
        Subscription::none()
    }

    pub fn update<M>(&mut self, _message: LocaleMessage) -> Command<M> {
        Command::none()
    }

    pub fn view(&self) -> Element<LocaleMessage> {
        let zebra_print = zebra_ui::image::zebra_print_view();
        let title = Text::new(t!("welcome"))
            .size(24)
            .horizontal_alignment(Horizontal::Center);
        let btn = Button::new(
            Text::new(t!("next"))
                .size(20)
                .horizontal_alignment(Horizontal::Center),
        )
        .style(zebra_ui::theme::Button::OutlinePrimary.into())
        .on_press(LocaleMessage::Next)
        .width(120);

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
            .push(btn);
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
