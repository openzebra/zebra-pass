//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::widget::container::Style;
use iced::{Border, Color, Theme};

use crate::config::BORDER_RADIUS;

pub fn primary_bordered_hover(theme: &Theme) -> Style {
    let palette = theme.extended_palette();
    Style {
        background: Some(Color::TRANSPARENT.into()),
        border: Border {
            width: 1.0,
            radius: BORDER_RADIUS.into(),
            color: palette.primary.weak.color,
        },
        ..Style::default()
    }

    // match status {
    //     Status::Idle => Style {
    //         background: Some(Color::TRANSPARENT.into()),
    //         border: Border {
    //             width: 1.0,
    //             radius: BORDER_RADIUS.into(),
    //             color: palette.primary.weak.color,
    //         },
    //         ..Style::default()
    //     },
    //     Status::Hovered => Style {
    //         background: Some(Color::TRANSPARENT.into()),
    //         border: Border {
    //             width: 1.0,
    //             radius: BORDER_RADIUS.into(),
    //             color: palette.primary.strong.color,
    //         },
    //         ..Style::default()
    //     },
    // }
}

pub fn primary_bordered_disabled(theme: &Theme) -> Style {
    let palette = theme.extended_palette();
    let mut color = palette.primary.weak.color;

    color.a = 0.5;

    Style {
        background: Some(Color::TRANSPARENT.into()),
        border: Border {
            color,
            width: 1.0,
            radius: BORDER_RADIUS.into(),
        },
        ..Style::default()
    }
}

pub fn primary_bordered(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Some(Color::TRANSPARENT.into()),
        border: Border {
            width: 1.0,
            radius: BORDER_RADIUS.into(),
            color: palette.primary.strong.color,
        },
        ..Style::default()
    }
}

pub fn danger_bordered_hover(theme: &Theme) -> Style {
    let palette = theme.extended_palette();
    Style {
        background: Some(Color::TRANSPARENT.into()),
        border: Border {
            width: 1.0,
            radius: BORDER_RADIUS.into(),
            color: palette.danger.weak.color,
        },
        ..Style::default()
    }

    // match status {
    //     Status::Idle => Style {
    //         background: Some(Color::TRANSPARENT.into()),
    //         border: Border {
    //             width: 1.0,
    //             radius: BORDER_RADIUS.into(),
    //             color: palette.danger.weak.color,
    //         },
    //         ..Style::default()
    //     },
    //     Status::Hovered => Style {
    //         background: Some(Color::TRANSPARENT.into()),
    //         border: Border {
    //             width: 1.0,
    //             radius: BORDER_RADIUS.into(),
    //             color: palette.danger.strong.color,
    //         },
    //         ..Style::default()
    //     },
    // }
}

pub fn primary_bordered_modal(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: Some(palette.background.base.color.into()),
        border: Border {
            width: 1.0,
            radius: BORDER_RADIUS.into(),
            color: palette.primary.strong.color,
        },
        ..Style::default()
    }
}

pub fn danger_bordered_disabled(theme: &Theme) -> Style {
    let palette = theme.extended_palette();
    let mut color = palette.danger.base.color;

    color.a = 0.5;

    Style {
        background: Some(Color::TRANSPARENT.into()),
        border: Border {
            color,
            width: 1.0,
            radius: BORDER_RADIUS.into(),
        },
        ..Style::default()
    }
}
