//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use crate::rust_i18n::t;
use iced::{
    alignment::Horizontal,
    widget::{combo_box, pick_list, ComboBox, Space},
    Alignment, Command, Length, Subscription,
};
use zebra_lib::core::core::Core;
use zebra_ui::widget::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Language {
    Russian(String),
    English(String),
}

#[derive(Debug)]
pub struct Locale {
    locales: [Language; 2],
    selected: Option<Language>,
}

#[derive(Debug, Clone)]
pub enum LocaleMessage {
    Next,
    Selected(Language),
}

impl Locale {
    pub fn new() -> Self {
        let locales = [
            Language::Russian("ru".to_string()),
            Language::English("en".to_string()),
        ];
        let selected = Some(locales[0].clone());

        Self { locales, selected }
    }

    pub fn subscription(&self) -> Subscription<LocaleMessage> {
        Subscription::none()
    }

    pub fn update<M>(&mut self, message: LocaleMessage, core: &mut Core) -> Command<M> {
        match message {
            LocaleMessage::Next => Command::none(),
            LocaleMessage::Selected(lang) => {
                self.selected = Some(lang.clone());
                let s = lang.symbol();

                rust_i18n::set_locale(&s);
                core.state.borrow_mut().payload.settings.locale = s;
                core.state.borrow_mut().update().unwrap(); // TODO: Remove unwrap!

                Command::none()
            }
        }
    }

    pub fn view(&self) -> Element<LocaleMessage> {
        let locale_pick_list: iced::widget::PickList<'_, Language, LocaleMessage, Renderer> =
            pick_list(
                self.locales.as_slice(),
                self.selected.clone(),
                LocaleMessage::Selected,
            )
            .text_size(20)
            .padding(5)
            .width(220)
            .style(zebra_ui::theme::PickList::OutlineLight);

        let zebra_print = zebra_ui::image::zebra_print_view();
        let title = Text::new(t!("welcome"))
            .size(34)
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
            .push(Space::new(0, 20))
            .push(locale_pick_list)
            .push(Space::new(0, 200))
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

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::Russian(s) => t!(&format!("locale.{s}")),
                Language::English(s) => t!(&format!("locale.{s}")),
            }
        )
    }
}

impl Language {
    pub fn symbol(&self) -> String {
        match self {
            Language::Russian(s) => s.to_owned(),
            Language::English(s) => s.to_owned(),
        }
    }
}
