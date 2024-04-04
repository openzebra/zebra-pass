//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::{
    widget::{
        container,
        scrollable::{Appearance, Scrollbar, Scroller, Status},
    },
    Color, Theme,
};

pub fn scroll_transparent(_theme: &Theme, _status: Status) -> Appearance {
    let scrollbar = Scrollbar {
        background: Some(Color::TRANSPARENT.into()),
        border: Default::default(),
        scroller: Scroller {
            color: Color::TRANSPARENT,
            border: Default::default(),
        },
    };

    Appearance {
        container: container::Appearance::default(),
        vertical_scrollbar: scrollbar,
        horizontal_scrollbar: scrollbar,
        gap: None,
    }
}
