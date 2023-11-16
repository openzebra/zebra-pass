//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::{
    theme,
    widget::svg::{self, Handle, Svg},
    ContentFit, Length,
};

const PRINT: &[u8] = include_bytes!("../../../public/imgs/zebra_print.svg");

pub fn zebra_print_view() -> Svg {
    let h = Handle::from_memory(PRINT);
    Svg::new(h)
        .height(Length::Fill)
        .height(Length::Fill)
        .content_fit(ContentFit::Cover)
        .style(theme::Svg::custom_fn(|theme| svg::Appearance {
            color: Some(theme.palette().text),
        }))
}
