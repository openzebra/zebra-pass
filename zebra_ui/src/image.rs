//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use iced::{ContentFit, Length};

use crate::{style::svg, widget::Svg};
use iced::{widget::svg::Handle, window::icon};

const PRINT: &[u8] = include_bytes!("../static/imgs/zebra_print.svg");
const ZEBRAS_HEAT: &[u8] = include_bytes!("../static/imgs/zebra.svg");
const ATOM: &[u8] = include_bytes!("../static/imgs/atom.svg");
const APP_ICON: &[u8] = include_bytes!("../static/imgs/logo.webp");

pub fn zebra_print_view() -> Svg {
    let h = Handle::from_memory(PRINT);
    Svg::new(h)
        .height(Length::Fill)
        .height(Length::Fill)
        .content_fit(ContentFit::Cover)
        .style(svg::Svg::Inverse)
}

pub fn zebra_app_icon() -> icon::Icon {
    icon::from_file_data(APP_ICON, None).unwrap()
}

pub fn zebra_heat() -> Svg {
    let h = Handle::from_memory(ZEBRAS_HEAT);
    Svg::new(h)
        .height(Length::Fill)
        .height(Length::Fill)
        .content_fit(ContentFit::Cover)
        .style(svg::Svg::Inverse)
}

pub fn atom() -> Svg {
    let h = Handle::from_memory(ATOM);
    Svg::new(h)
        .height(Length::Fill)
        .height(Length::Fill)
        .content_fit(ContentFit::Cover)
        .style(svg::Svg::Inverse)
}
