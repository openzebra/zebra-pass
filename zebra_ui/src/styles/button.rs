//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::{
    widget::button::{Appearance, Status},
    Theme,
};

pub fn transparent(_theme: &Theme, status: Status) -> Appearance {
    match status {
        _ => Appearance {
            background: None,
            ..Default::default()
        },
    }
}
