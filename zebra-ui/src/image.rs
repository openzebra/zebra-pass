//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::{theme, ContentFit, Length};

use crate::widget::Svg;
use iced::{widget::svg::Handle, window::icon};

const PRINT: &[u8] = include_bytes!("../static/imgs/zebra_print.svg");
const APP_ICON: &[u8] = include_bytes!("../static/imgs/logo.webp");

pub fn zebra_print_view() -> Svg {
    let h = Handle::from_memory(PRINT);
    Svg::new(h)
        .height(Length::Fill)
        .height(Length::Fill)
        .content_fit(ContentFit::Cover)
    // TODO: make it auto palitre
    // .style(theme::Svg::custom_fn(|theme| Svg::Appearance {
    //     color: Some(theme.palette().text),
    // }))
}

pub fn liana_app_icon() -> icon::Icon {
    icon::from_file_data(APP_ICON, None).unwrap()
}