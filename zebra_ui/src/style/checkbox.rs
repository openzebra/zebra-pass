//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::Theme;
use iced::{widget::checkbox, BorderRadius};

#[derive(Default)]
pub struct CheckBox {}
impl checkbox::StyleSheet for Theme {
    type Style = CheckBox;

    fn active(&self, _style: &Self::Style, is_selected: bool) -> checkbox::Appearance {
        let palette = match self {
            Theme::Dark(p) => p,
            Theme::Light(p) => p,
        };

        if is_selected {
            checkbox::Appearance {
                background: palette.window_background_inverse.into(),
                border_width: 0.0,
                border_color: iced::Color::TRANSPARENT,
                icon_color: palette.window_background_inverse,
                text_color: None,
                border_radius: BorderRadius::from(palette.radius),
            }
        } else {
            checkbox::Appearance {
                background: palette.danger.into(),
                border_width: 0.0,
                border_color: iced::Color::TRANSPARENT,
                icon_color: palette.window_background_inverse,
                text_color: None,
                border_radius: BorderRadius::from(palette.radius),
            }
        }
    }

    fn hovered(&self, style: &Self::Style, is_selected: bool) -> checkbox::Appearance {
        self.active(style, is_selected)
    }
}
