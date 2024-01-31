//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::Theme;
use iced::{widget::container, Border, Color};

#[derive(Debug, Copy, Clone, Default)]
pub enum Container {
    #[default]
    Transparent,
    Background,
    Bordered,
    WeekBorder,
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
                Container::Bordered => container::Appearance {
                    text_color: Default::default(),
                    background: Some(iced::Background::Color(Color::TRANSPARENT)),
                    shadow: Default::default(),
                    border: Border {
                        radius: 16.0.into(),
                        width: 2.0,
                        color: p.window_background_inverse.into(),
                    },
                },
                Container::WeekBorder => container::Appearance {
                    text_color: Default::default(),
                    background: Some(iced::Background::Color(Color::TRANSPARENT)),
                    border: Border {
                        radius: 0.0.into(),
                        width: 0.5,
                        color: p.secondary.into(),
                    },
                    shadow: Default::default(),
                },
                Container::Custom(c) => container::Appearance {
                    background: Some(iced::Background::Color(*c)),
                    ..container::Appearance::default()
                },
            },
            Theme::Dark(p) => match style {
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
                Container::Bordered => container::Appearance {
                    text_color: Default::default(),
                    shadow: Default::default(),
                    background: Some(iced::Background::Color(Color::TRANSPARENT)),
                    border: Border {
                        radius: 16.0.into(),
                        width: 2.0,
                        color: p.window_background_inverse.into(),
                    },
                },
                Container::WeekBorder => container::Appearance {
                    text_color: Default::default(),
                    shadow: Default::default(),
                    background: Some(iced::Background::Color(Color::TRANSPARENT)),
                    border: Border {
                        radius: 0.0.into(),
                        width: 1.0,
                        color: p.secondary.into(),
                    },
                },
            },
        }
    }
}
