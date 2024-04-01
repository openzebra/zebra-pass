//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::{
    widget::button::{Appearance, Status},
    Border, Color, Theme,
};

use crate::config::BORDER_RADIUS;

pub fn transparent(_theme: &Theme, status: Status) -> Appearance {
    match status {
        _ => Appearance {
            background: None,
            ..Default::default()
        },
    }
}

pub fn outline_primary(theme: &Theme, status: Status) -> Appearance {
    let palette = theme.extended_palette();
    let base = Appearance {
        background: Some(Color::TRANSPARENT.into()),
        text_color: palette.background.base.text,
        border: Border {
            color: palette.primary.base.color,
            width: 1.0,
            radius: BORDER_RADIUS.into(),
        },
        shadow: Default::default(),
    };

    match status {
        Status::Active => Appearance { ..base },
        Status::Pressed => Appearance {
            background: Some(palette.primary.weak.color.into()),
            text_color: palette.background.base.color,
            border: Border {
                color: palette.primary.base.color,
                width: 1.0,
                radius: BORDER_RADIUS.into(),
            },
            ..base
        },
        Status::Hovered => Appearance {
            background: Some(palette.primary.base.color.into()),
            text_color: palette.background.base.color,
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

pub fn ref_primary(theme: &Theme, status: Status) -> Appearance {
    let palette = theme.extended_palette();
    let base = Appearance {
        background: Some(Color::TRANSPARENT.into()),
        text_color: palette.primary.strong.color,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        shadow: Default::default(),
    };

    match status {
        Status::Active => Appearance { ..base },
        Status::Pressed => Appearance {
            text_color: palette.primary.weak.color,
            ..base
        },
        Status::Hovered => Appearance {
            text_color: palette.primary.weak.color,
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

fn disabled(appearance: Appearance) -> Appearance {
    Appearance {
        background: appearance
            .background
            .map(|background| background.scale_alpha(0.5)),
        text_color: appearance.text_color.scale_alpha(0.5),
        ..appearance
    }
}
