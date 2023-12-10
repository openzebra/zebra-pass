//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::Theme;
use iced::widget::svg;

#[derive(Debug, Copy, Clone, Default)]
pub enum Svg {
    #[default]
    Primary,
    Inverse,
    Normal,
}

impl svg::StyleSheet for Theme {
    type Style = Svg;

    fn appearance(&self, style: &Self::Style) -> svg::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        match style {
            Svg::Normal => svg::Appearance::default(),
            Svg::Primary => svg::Appearance {
                color: Some(palette.primary),
            },
            Svg::Inverse => svg::Appearance {
                color: Some(palette.window_background_inverse),
            },
        }
    }

    fn hovered(&self, style: &Self::Style) -> svg::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };
        match style {
            Svg::Normal => svg::Appearance::default(),
            Svg::Primary => svg::Appearance {
                color: Some(palette.window_background_inverse),
            },
            Svg::Inverse => svg::Appearance {
                color: Some(palette.window_background_inverse),
            },
        }
    }
}
