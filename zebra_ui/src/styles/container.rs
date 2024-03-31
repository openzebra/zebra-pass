//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::widget::container::{Appearance, Status};
use iced::{Border, Color, Theme};

use crate::config::BORDER_RADIUS;

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
