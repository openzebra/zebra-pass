//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::Theme;
use iced::widget::button;

#[derive(Default)]
pub enum Button {
    #[default]
    Primary,
    OutlinePrimary,
    Transparent,
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        let background_color = match style {
            Button::Primary => palette.primary,
            Button::OutlinePrimary => iced::Color::TRANSPARENT,
            Button::Transparent => iced::Color::TRANSPARENT,
        };
        let border_color = match style {
            Button::Primary => iced::Color::TRANSPARENT,
            Button::OutlinePrimary => palette.primary,
            Button::Transparent => iced::Color::TRANSPARENT,
        };
        let border_width = match style {
            Button::Primary => 1.0,
            Button::OutlinePrimary => 1.0,
            Button::Transparent => 1.0,
        };
        let border_radius = match style {
            Button::Primary => 6.0.into(),
            Button::OutlinePrimary => 6.0.into(),
            Button::Transparent => 100.0.into(),
        };
        let text_color = match style {
            Button::Primary => palette.window_background_inverse,
            Button::OutlinePrimary => palette.primary,
            Button::Transparent => iced::Color::TRANSPARENT,
        };

        iced::widget::button::Appearance {
            text_color,
            border_color,
            border_width,
            border_radius,
            background: Some(background_color.into()),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        let background_color = match style {
            Button::Primary => iced::Color::TRANSPARENT,
            Button::OutlinePrimary => palette.primary,
            Button::Transparent => iced::Color::TRANSPARENT,
        };
        let border_color = match style {
            Button::Primary => palette.primary,
            Button::OutlinePrimary => palette.primary,
            Button::Transparent => iced::Color::TRANSPARENT,
        };
        let border_width = match style {
            Button::Primary => 1.0,
            Button::OutlinePrimary => 1.0,
            Button::Transparent => 1.0,
        };
        let border_radius = match style {
            Button::Primary => 6.0.into(),
            Button::OutlinePrimary => 6.0.into(),
            Button::Transparent => 100.0.into(),
        };
        let text_color = match style {
            Button::Primary => palette.primary,
            Button::OutlinePrimary => palette.window_background_inverse,
            Button::Transparent => iced::Color::TRANSPARENT,
        };

        iced::widget::button::Appearance {
            text_color,
            border_color,
            border_width,
            border_radius,
            background: Some(background_color.into()),
            ..Default::default()
        }
    }
}
