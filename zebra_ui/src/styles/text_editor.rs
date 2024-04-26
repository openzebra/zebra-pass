//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::widget::text_editor::{Appearance, Status};
use iced::{Border, Color, Theme};

use crate::config::BORDER_RADIUS;

pub fn primary(theme: &Theme, status: Status) -> Appearance {
    let palette = theme.extended_palette();

    let active = Appearance {
        background: Color::TRANSPARENT.into(),
        border: Border {
            radius: BORDER_RADIUS.into(),
            width: 1.0,
            color: palette.primary.base.color,
        },
        icon: palette.primary.weak.text,
        placeholder: palette.primary.strong.color,
        value: palette.primary.base.text,
        selection: palette.primary.weak.color,
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
        Status::Focused => Appearance {
            border: Border {
                color: palette.primary.strong.color,
                ..active.border
            },
            ..active
        },
        Status::Disabled => Appearance {
            background: Color::TRANSPARENT.into(),
            value: active.placeholder,
            ..active
        },
    }
}
