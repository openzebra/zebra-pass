//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use crate::components::line::Appearance;
use iced::{Color, Theme};

pub fn line_inverse(theme: &Theme) -> Appearance {
    let palette = theme.extended_palette();

    Appearance {
        bar_color: palette.background.base.text,
    }
}

pub fn line_secondary(theme: &Theme) -> Appearance {
    let palette = theme.extended_palette();

    Appearance {
        bar_color: palette.secondary.base.color,
    }
}

pub fn line_transparent(_theme: &Theme) -> Appearance {
    Appearance {
        bar_color: Color::TRANSPARENT,
    }
}
