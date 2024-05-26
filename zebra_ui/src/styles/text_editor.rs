//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::widget::text_editor::{Status, Style};
use iced::{Border, Color, Theme};

use crate::config::BORDER_RADIUS;

pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let active = Style {
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
        Status::Hovered => Style {
            border: Border {
                color: palette.primary.strong.color,
                ..active.border
            },
            ..active
        },
        Status::Focused => Style {
            border: Border {
                color: palette.primary.strong.color,
                ..active.border
            },
            ..active
        },
        Status::Disabled => Style {
            background: Color::TRANSPARENT.into(),
            value: active.placeholder,
            ..active
        },
    }
}
