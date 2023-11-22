//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::Theme;
use iced::{widget::container, Color};

#[derive(Debug, Copy, Clone, Default)]
pub enum Container {
    #[default]
    Transparent,
    Background,
    Custom(iced::Color),
}

impl container::StyleSheet for Theme {
    type Style = Container;
    fn appearance(&self, style: &Self::Style) -> iced::widget::container::Appearance {
        match self {
            Theme::Light(p) => match style {
                Container::Transparent => container::Appearance {
                    background: Some(iced::Background::Color(Color::TRANSPARENT)),
                    ..container::Appearance::default()
                },
                Container::Background => container::Appearance {
                    background: Some(iced::Background::Color(p.danger)),
                    ..container::Appearance::default()
                },
                Container::Custom(c) => container::Appearance {
                    background: Some(iced::Background::Color(*c)),
                    ..container::Appearance::default()
                },
            },
            Theme::Dark(_p) => match style {
                Container::Transparent => container::Appearance {
                    background: Some(iced::Background::Color(Color::TRANSPARENT)),
                    ..container::Appearance::default()
                },
                Container::Background => container::Appearance {
                    background: Some(iced::Background::Color(Color::TRANSPARENT)),
                    ..container::Appearance::default()
                },
                Container::Custom(c) => container::Appearance {
                    background: Some(iced::Background::Color(*c)),
                    ..container::Appearance::default()
                },
            },
        }
    }
}
