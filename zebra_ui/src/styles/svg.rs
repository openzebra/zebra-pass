//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::{
    widget::svg::{Status, Style},
    Theme,
};

pub fn primary_hover(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    match status {
        Status::Idle => Style {
            color: Some(palette.primary.strong.color),
        },
        Status::Hovered => Style {
            color: Some(palette.primary.weak.color),
        },
    }
}

pub fn primary_disabled(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let mut color = palette.primary.weak.color;

    color.a = 0.5;

    match status {
        Status::Idle => Style { color: Some(color) },
        Status::Hovered => Style { color: Some(color) },
    }
}

pub fn bg_inverse(theme: &Theme, _status: Status) -> Style {
    let palette = theme.extended_palette();

    Style {
        color: Some(palette.background.base.text),
    }
}
