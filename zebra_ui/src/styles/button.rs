//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::{
    widget::button::{Status, Style},
    Border, Color, Theme,
};

use crate::config::BORDER_RADIUS;

pub fn transparent(_theme: &Theme, _status: Status) -> Style {
    Style {
        background: None,
        ..Default::default()
    }
}

pub fn primary_rude(theme: &Theme, status: Status) -> Style {
    let mut base = primary(theme, status);

    base.border.radius = 0.0.into();

    Style { ..base }
}

pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = Style {
        background: Some(palette.primary.strong.color.into()),
        text_color: palette.background.base.text,
        border: Border {
            color: palette.primary.weak.color,
            width: 1.0,
            radius: BORDER_RADIUS.into(),
        },
        shadow: Default::default(),
    };

    match status {
        Status::Active => Style { ..base },
        Status::Pressed => Style {
            background: Some(palette.primary.strong.color.into()),
            text_color: palette.background.base.text,
            border: Border {
                color: palette.primary.weak.color,
                width: 1.0,
                radius: BORDER_RADIUS.into(),
            },
            ..base
        },
        Status::Hovered => Style {
            background: Some(palette.primary.strong.color.into()),
            text_color: palette.background.base.text,
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

pub fn outline_primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = Style {
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
        Status::Active => Style { ..base },
        Status::Pressed => Style {
            background: Some(palette.primary.weak.color.into()),
            text_color: palette.background.base.color,
            border: Border {
                color: palette.primary.base.color,
                width: 1.0,
                radius: BORDER_RADIUS.into(),
            },
            ..base
        },
        Status::Hovered => Style {
            background: Some(palette.primary.base.color.into()),
            text_color: palette.background.base.color,
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

pub fn ref_primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = Style {
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
        Status::Active => Style { ..base },
        Status::Pressed => Style {
            text_color: palette.primary.weak.color,
            ..base
        },
        Status::Hovered => Style {
            text_color: palette.primary.weak.color,
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

pub fn ref_danger(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = Style {
        background: Some(Color::TRANSPARENT.into()),
        text_color: palette.danger.strong.color,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        shadow: Default::default(),
    };

    match status {
        Status::Active => Style { ..base },
        Status::Pressed => Style {
            text_color: palette.danger.weak.color,
            ..base
        },
        Status::Hovered => Style {
            text_color: palette.danger.weak.color,
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

pub fn outline_danger(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = Style {
        background: Some(Color::TRANSPARENT.into()),
        text_color: palette.danger.base.color,
        border: Border {
            color: palette.danger.weak.color,
            width: 1.0,
            radius: BORDER_RADIUS.into(),
        },
        shadow: Default::default(),
    };

    match status {
        Status::Active => Style { ..base },
        Status::Pressed => Style {
            background: Some(palette.danger.weak.color.into()),
            text_color: palette.background.base.text,
            border: Border {
                color: palette.danger.base.color,
                width: 1.0,
                radius: BORDER_RADIUS.into(),
            },
            ..base
        },
        Status::Hovered => Style {
            background: Some(palette.danger.base.color.into()),
            text_color: palette.background.base.text,
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

fn disabled(appearance: Style) -> Style {
    Style {
        background: appearance
            .background
            .map(|background| background.scale_alpha(0.5)),
        text_color: appearance.text_color.scale_alpha(0.5),
        ..appearance
    }
}
