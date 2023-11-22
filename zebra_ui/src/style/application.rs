//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::Theme;
use iced::application;

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        match self {
            Theme::Light(palette) => application::Appearance {
                background_color: palette.window_background,
                text_color: palette.window_background_inverse,
            },
            Theme::Dark(palette) => application::Appearance {
                background_color: palette.window_background,
                text_color: palette.window_background_inverse,
            },
        }
    }
}
