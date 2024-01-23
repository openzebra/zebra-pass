//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::Theme;
use iced::widget::checkbox;
use iced::Border;

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
                background: iced::Color::TRANSPARENT.into(),
                icon_color: palette.primary,
                text_color: None,
                border: Border {
                    radius: 6.0,
                    color: palette.primary,
                    width: 1.0,
                },
            }
        } else {
            checkbox::Appearance {
                background: palette.window_background.into(),
                icon_color: palette.primary,
                text_color: None,
                border: Border {
                    radius: 6.0,
                    color: palette.primary,
                    width: 1.0,
                },
            }
        }
    }

    fn hovered(&self, style: &Self::Style, is_selected: bool) -> checkbox::Appearance {
        self.active(style, is_selected)
    }
}
