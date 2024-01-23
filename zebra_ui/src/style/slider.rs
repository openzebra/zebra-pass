//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::Theme;
use iced::widget::slider;

#[derive(Debug, Copy, Clone, Default)]
pub enum Slider {
    #[default]
    Primary,
}

impl slider::StyleSheet for Theme {
    type Style = Slider;
    fn active(&self, style: &Self::Style) -> slider::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        let handle = slider::Handle {
            shape: slider::HandleShape::Rectangle {
                width: 8,
                border_radius: 4.0.into(),
            },
            color: palette.window_background_inverse,
            border_width: 1.0,
            border_color: palette.window_background_inverse,
        };

        match style {
            Slider::Primary => slider::Appearance {
                rail: slider::Rail {
                    colors: (palette.primary, iced::Color::TRANSPARENT),
                    width: 2.0,
                    border_radius: 19.0.into(),
                },
                handle,
            },
        }
    }
    fn hovered(&self, style: &Self::Style) -> slider::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        let handle = slider::Handle {
            shape: slider::HandleShape::Rectangle {
                width: 8,
                border_radius: 4.0.into(),
            },
            color: palette.primary,
            border_color: palette.window_background_inverse,
            border_width: 1.0,
        };

        match style {
            Slider::Primary => slider::Appearance {
                rail: slider::Rail {
                    colors: (palette.primary, iced::Color::TRANSPARENT),
                    width: 2.0,
                    border_radius: 19.0.into(),
                },
                handle,
            },
        }
    }
    fn dragging(&self, style: &Self::Style) -> slider::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        let handle = slider::Handle {
            shape: slider::HandleShape::Rectangle {
                width: 8,
                border_radius: 4.0.into(),
            },
            color: palette.primary,
            border_color: palette.secondary,
            border_width: 1.0,
        };

        match style {
            Slider::Primary => slider::Appearance {
                rail: slider::Rail {
                    colors: (palette.primary, iced::Color::TRANSPARENT),
                    width: 2.0,
                    border_radius: 4.0.into(),
                },
                handle,
            },
        }
    }
}
