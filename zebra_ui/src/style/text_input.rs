//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::Theme;
use iced::{widget::text_input, BorderRadius};

#[derive(Debug, Copy, Clone, Default)]
pub enum TextInput {
    #[default]
    Primary,
}

impl text_input::StyleSheet for Theme {
    type Style = TextInput;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        let border_radius = BorderRadius::from(palette.radius);
        let border_width = 1.0;

        match style {
            TextInput::Primary => {
                let mut alfa_primary = palette.primary;

                alfa_primary.a = 0.8;

                text_input::Appearance {
                    border_width,
                    border_radius,
                    icon_color: palette.primary,
                    background: iced::Background::Color(iced::Color::TRANSPARENT),
                    border_color: alfa_primary,
                }
            }
        }
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            ..self.active(style)
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        let border_radius = BorderRadius::from(palette.radius);
        let border_width = 1.0;

        match style {
            TextInput::Primary => text_input::Appearance {
                border_width,
                border_radius,
                icon_color: palette.primary,
                background: iced::Background::Color(iced::Color::TRANSPARENT),
                border_color: palette.primary,
            },
        }
    }

    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        palette.primary
    }

    fn placeholder_color(&self, style: &Self::Style) -> iced::Color {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        match style {
            TextInput::Primary => {
                let mut alfa_primary = palette.primary;
                alfa_primary.a = 0.1;

                alfa_primary
            }
        }
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        palette.window_background_inverse
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        palette.window_background_inverse
    }
}
