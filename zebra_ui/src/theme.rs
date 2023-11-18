//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use iced::{
    application,
    widget::{
        button, checkbox, container, pick_list, progress_bar, radio, scrollable, slider, svg, text,
        text_input,
    },
};

use super::color::ZebraPalette;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        match self {
            Theme::Light => application::Appearance {
                background_color: ZebraPalette::LIGHT.window_background,
                text_color: ZebraPalette::LIGHT.window_background_inverse,
            },
            Theme::Dark => application::Appearance {
                background_color: ZebraPalette::DARK.window_background,
                text_color: ZebraPalette::DARK.window_background_inverse,
            },
        }
    }
}
