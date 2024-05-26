//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use iced::{overlay::menu::Style, Border, Theme};

use crate::config::BORDER_RADIUS;

pub fn primary_menu(theme: &Theme) -> Style {
    let palette = theme.extended_palette();

    Style {
        background: palette.background.base.color.into(),
        border: Border {
            width: 1.0,
            radius: [0.0, 0.0, BORDER_RADIUS, BORDER_RADIUS].into(),
            color: palette.primary.base.color,
        },
        text_color: palette.background.weak.text,
        selected_text_color: palette.primary.strong.text,
        selected_background: palette.primary.strong.color.into(),
    }
}
