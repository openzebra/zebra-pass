//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::Theme;
use iced::{widget::text_input, BorderRadius};

#[derive(Debug, Copy, Clone, Default)]
pub enum Form {
    #[default]
    Simple,
    Invalid,
}

impl text_input::StyleSheet for Theme {
    type Style = Form;
    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        let border_radius = BorderRadius::from(18.0);
        let border_width = 1.0;
        match style {
            Form::Simple => text_input::Appearance {
                border_width,
                border_radius,
                icon_color: palette.info,
                background: iced::Background::Color(iced::Color::TRANSPARENT),
                border_color: palette.primary,
            },
            Form::Invalid => text_input::Appearance {
                border_radius,
                border_width,
                icon_color: palette.info,
                background: iced::Background::Color(iced::Color::TRANSPARENT),
                border_color: palette.secondary,
            },
        }
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            ..self.active(style)
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            ..self.active(style)
        }
    }

    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        palette.primary
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        palette.secondary
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        palette.danger
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        palette.warn
    }
}
