//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::{
    widget::svg::{Appearance, Status},
    Theme,
};

pub fn primary_hover(theme: &Theme, status: Status) -> Appearance {
    let palette = theme.extended_palette();

    match status {
        Status::Idle => Appearance {
            color: Some(palette.primary.strong.color),
        },
        Status::Hovered => Appearance {
            color: Some(palette.primary.weak.color),
        },
    }
}

pub fn primary_disabled(theme: &Theme, status: Status) -> Appearance {
    let palette = theme.extended_palette();
    let mut color = palette.primary.weak.color;

    color.a = 0.5;

    match status {
        Status::Idle => Appearance { color: Some(color) },
        Status::Hovered => Appearance { color: Some(color) },
    }
}

pub fn bg_inverse(theme: &Theme, _status: Status) -> Appearance {
    let palette = theme.extended_palette();

    Appearance {
        color: Some(palette.background.base.text),
    }
}
