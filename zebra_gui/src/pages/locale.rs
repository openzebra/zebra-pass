//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use crate::rust_i18n::t;
use iced::{
    alignment::Horizontal,
    widget::{combo_box, pick_list, ComboBox},
    Alignment, Command, Length, Subscription,
};
use zebra_ui::widget::*;

#[derive(Debug)]
pub struct Locale {
    locales: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum LocaleMessage {
    Chose,
    Next,
    Selected(String),
}

impl Locale {
    pub fn new() -> Self {
        let locales = rust_i18n::available_locales!()
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<String>>();

        Self { locales }
    }

    pub fn subscription(&self) -> Subscription<LocaleMessage> {
        Subscription::none()
    }

    pub fn update<M>(&mut self, _message: LocaleMessage) -> Command<M> {
        Command::none()
    }

    pub fn view(&self) -> Element<LocaleMessage> {
        let locale_pick_list: iced::widget::PickList<'_, String, LocaleMessage, Renderer> =
            pick_list(
                &self.locales,
                Some("en".to_string()),
                LocaleMessage::Selected,
            )
            .style(zebra_ui::theme::PickList::Primary);

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
            .push(locale_pick_list)
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
