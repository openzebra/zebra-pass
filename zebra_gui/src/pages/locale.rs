//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use iced::{widget::text, Alignment, Command, Length, Subscription};
use zebra_ui::widget::*;

#[derive(Debug)]
pub struct Locale {
    available_locales: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum LocaleMessage {}

impl Locale {
    pub fn new() -> Self {
        let available_locales = rust_i18n::available_locales!()
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<String>>();

        Self { available_locales }
    }

    pub fn stop(&self) {}

    pub fn subscription(&self) -> Subscription<LocaleMessage> {
        Subscription::none()
    }

    pub fn update<M>(&mut self, _message: LocaleMessage) -> Command<M> {
        Command::none()
    }

    pub fn view(&self) -> Element<LocaleMessage> {
        text("test").size(20).into()
    }
}
