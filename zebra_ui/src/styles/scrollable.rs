//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::{
    widget::{
        container,
        scrollable::{Scrollbar, Scroller, Status, Style},
    },
    Color, Theme,
};

pub fn scroll_transparent(_theme: &Theme, _status: Status) -> Style {
    let scrollbar = Scrollbar {
        background: Some(Color::TRANSPARENT.into()),
        border: Default::default(),
        scroller: Scroller {
            color: Color::TRANSPARENT,
            border: Default::default(),
        },
    };

    Style {
        container: container::Style::default(),
        vertical_scrollbar: scrollbar,
        horizontal_scrollbar: scrollbar,
        gap: None,
    }
}
