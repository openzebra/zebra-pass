//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::Theme;
use iced::widget::radio;

#[derive(Default)]
pub struct Radio {} // TODO: add primary, secondary...
impl radio::StyleSheet for Theme {
    type Style = Radio;

    fn active(&self, _style: &Self::Style, _is_selected: bool) -> radio::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        radio::Appearance {
            background: iced::Color::TRANSPARENT.into(),
            dot_color: palette.primary,
            border_width: 1.0,
            border_color: palette.primary,
            text_color: None,
        }
    }

    fn hovered(&self, style: &Self::Style, is_selected: bool) -> radio::Appearance {
        let active = self.active(style, is_selected);
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        radio::Appearance {
            dot_color: palette.secondary,
            background: iced::Color::TRANSPARENT.into(),
            ..active
        }
    }
}
