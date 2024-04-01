//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::widget::text_input::{Appearance, Status};
use iced::{Background, Border, Color, Theme};

use crate::config::BORDER_RADIUS;

pub fn primary(theme: &Theme, status: Status) -> Appearance {
    let palette = theme.extended_palette();

    let active = Appearance {
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            radius: BORDER_RADIUS.into(),
            width: 1.0,
            color: palette.primary.weak.color,
        },
        icon: palette.primary.weak.text,
        placeholder: palette.background.weak.color,
        value: palette.background.base.text,
        selection: palette.primary.weak.color,
    };

    match status {
        Status::Active => active,
        Status::Hovered => Appearance {
            border: Border {
                color: palette.primary.base.color,
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
            background: Background::Color(palette.primary.weak.color),
            value: active.placeholder,
            ..active
        },
    }
}

pub fn danger(theme: &Theme, status: Status) -> Appearance {
    let palette = theme.extended_palette();

    let active = Appearance {
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            radius: BORDER_RADIUS.into(),
            width: 1.0,
            color: palette.danger.strong.color,
        },
        icon: palette.danger.weak.text,
        placeholder: palette.danger.weak.color,
        value: palette.danger.base.color,
        selection: palette.danger.weak.color,
    };

    match status {
        Status::Active => active,
        Status::Hovered => Appearance {
            border: Border {
                color: palette.danger.base.text,
                ..active.border
            },
            ..active
        },
        Status::Focused => Appearance {
            border: Border {
                color: palette.danger.strong.color,
                ..active.border
            },
            ..active
        },
        Status::Disabled => Appearance {
            background: Background::Color(palette.danger.weak.color),
            value: active.placeholder,
            ..active
        },
    }
}

pub fn transparent_primary(theme: &Theme, _status: Status) -> Appearance {
    let palette = theme.extended_palette();
    let mut placeholder = palette.primary.weak.color;

    placeholder.a = 0.5;

    let active = Appearance {
        placeholder,
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        icon: Color::TRANSPARENT,
        value: palette.primary.base.color,
        selection: Color::TRANSPARENT,
    };

    active
}
