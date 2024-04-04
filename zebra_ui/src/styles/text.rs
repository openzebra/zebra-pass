//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::widget::text::Appearance;
use iced::Theme;

pub fn danger(theme: &Theme) -> Appearance {
    let palette = theme.extended_palette();

    Appearance {
        color: Some(palette.danger.weak.color),
    }
}
