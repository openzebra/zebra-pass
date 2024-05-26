//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::widget::text_input::{Status, Style};
use iced::{Background, Border, Color, Theme};

use crate::config::BORDER_RADIUS;

pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let active = Style {
        selection: disbaled_color(palette.primary.weak.color),
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            radius: BORDER_RADIUS.into(),
            width: 1.0,
            color: palette.primary.weak.color,
        },
        icon: palette.primary.weak.text,
        placeholder: palette.background.weak.color,
        value: palette.background.base.text,
    };

    match status {
        Status::Active => active,
        Status::Hovered => Style {
            border: Border {
                color: palette.primary.base.color,
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
            value: active.placeholder,
            border: Border {
                radius: BORDER_RADIUS.into(),
                width: 1.0,
                color: disbaled_color(palette.primary.weak.color),
            },
            ..active
        },
    }
}

pub fn danger(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let mut selection = palette.danger.weak.color;
    selection.a = 0.5;
    let active = Style {
        selection,
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            radius: BORDER_RADIUS.into(),
            width: 1.0,
            color: palette.danger.strong.color,
        },
        icon: palette.danger.weak.text,
        placeholder: palette.danger.weak.color,
        value: palette.danger.base.color,
    };

    match status {
        Status::Active => active,
        Status::Hovered => Style {
            border: Border {
                color: palette.danger.base.text,
                ..active.border
            },
            ..active
        },
        Status::Focused => Style {
            border: Border {
                color: palette.danger.strong.color,
                ..active.border
            },
            ..active
        },
        Status::Disabled => Style {
            background: Background::Color(palette.danger.weak.color),
            value: active.placeholder,
            ..active
        },
    }
}

pub fn transparent_primary(theme: &Theme, _status: Status) -> Style {
    let palette = theme.extended_palette();

    Style {
        selection: disbaled_color(palette.primary.weak.color),
        placeholder: disbaled_color(palette.primary.base.color),
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        icon: Color::TRANSPARENT,
        value: palette.primary.base.color,
    }
}

pub fn transparent_danger(theme: &Theme, _status: Status) -> Style {
    let palette = theme.extended_palette();
    Style {
        selection: disbaled_color(palette.danger.weak.color),
        placeholder: disbaled_color(palette.danger.base.color),
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            radius: 0.0.into(),
            width: 0.0,
            color: Color::TRANSPARENT,
        },
        icon: Color::TRANSPARENT,
        value: palette.danger.base.color,
    }
}

fn disbaled_color(mut color: Color) -> Color {
    color.a = 0.5;

    color
}
