//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use iced::{
    widget::pick_list::{Appearance, Status},
    Border, Theme,
};

use crate::config::BORDER_RADIUS;

pub fn primary_field(theme: &Theme, status: Status) -> Appearance {
    let palette = theme.extended_palette();

    let active = Appearance {
        text_color: palette.background.base.text,
        background: palette.background.base.color.into(),
        placeholder_color: palette.background.strong.color,
        handle_color: palette.background.weak.text,
        border: Border {
            radius: BORDER_RADIUS.into(),
            width: 1.0,
            color: palette.background.strong.color,
        },
    };

    match status {
        Status::Active => active,
        Status::Hovered => Appearance {
            border: Border {
                color: palette.primary.strong.color,
                ..active.border
            },
            ..active
        },
        Status::Opened => Appearance {
            border: Border {
                radius: [BORDER_RADIUS, BORDER_RADIUS, 0.0, 0.0].into(),
                color: palette.primary.strong.color,
                width: 1.0,
            },
            ..active
        },
    }
}
