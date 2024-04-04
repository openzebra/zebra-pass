//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::widget::container::{Appearance, Status};
use iced::{Border, Color, Theme};

use crate::config::BORDER_RADIUS;

pub fn primary_bordered_hover(theme: &Theme, status: Status) -> Appearance {
    let palette = theme.extended_palette();

    match status {
        Status::Idle => Appearance {
            background: Some(Color::TRANSPARENT.into()),
            border: Border {
                width: 1.0,
                radius: BORDER_RADIUS.into(),
                color: palette.primary.weak.color,
            },
            ..Appearance::default()
        },
        Status::Hovered => Appearance {
            background: Some(Color::TRANSPARENT.into()),
            border: Border {
                width: 1.0,
                radius: BORDER_RADIUS.into(),
                color: palette.primary.strong.color,
            },
            ..Appearance::default()
        },
    }
}

pub fn primary_bordered_disabled(theme: &Theme, _status: Status) -> Appearance {
    let palette = theme.extended_palette();
    let mut color = palette.primary.weak.color;

    color.a = 0.5;

    Appearance {
        background: Some(Color::TRANSPARENT.into()),
        border: Border {
            color,
            width: 1.0,
            radius: BORDER_RADIUS.into(),
        },
        ..Appearance::default()
    }
}

pub fn primary_bordered(theme: &Theme, _status: Status) -> Appearance {
    let palette = theme.extended_palette();

    Appearance {
        background: Some(Color::TRANSPARENT.into()),
        border: Border {
            width: 1.0,
            radius: BORDER_RADIUS.into(),
            color: palette.primary.strong.color,
        },
        ..Appearance::default()
    }
}

pub fn danger_bordered_hover(theme: &Theme, status: Status) -> Appearance {
    let palette = theme.extended_palette();

    match status {
        Status::Idle => Appearance {
            background: Some(Color::TRANSPARENT.into()),
            border: Border {
                width: 1.0,
                radius: BORDER_RADIUS.into(),
                color: palette.danger.weak.color,
            },
            ..Appearance::default()
        },
        Status::Hovered => Appearance {
            background: Some(Color::TRANSPARENT.into()),
            border: Border {
                width: 1.0,
                radius: BORDER_RADIUS.into(),
                color: palette.danger.strong.color,
            },
            ..Appearance::default()
        },
    }
}

pub fn danger_bordered_disabled(theme: &Theme, _status: Status) -> Appearance {
    let palette = theme.extended_palette();
    let mut color = palette.danger.base.color;

    color.a = 0.5;

    Appearance {
        background: Some(Color::TRANSPARENT.into()),
        border: Border {
            color,
            width: 1.0,
            radius: BORDER_RADIUS.into(),
        },
        ..Appearance::default()
    }
}
