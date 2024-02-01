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
const BACK_ICON: &[u8] = include_bytes!("../static/icons/back.svg");
const FORWARD_ICON: &[u8] = include_bytes!("../static/icons/forward.svg");
const RUST_ICON: &[u8] = include_bytes!("../static/imgs/rust.svg");
const HOOVES_ICON: &[u8] = include_bytes!("../static/icons/hooves.svg");
const RELOAD_ICON: &[u8] = include_bytes!("../static/icons/reload.svg");
const COPY_ICON: &[u8] = include_bytes!("../static/icons/copy.svg");
const LOCK_ICON: &[u8] = include_bytes!("../static/icons/lock.svg");
const ZEBRA_LOGO: &[u8] = include_bytes!("../static/imgs/zebra_logo.svg");

pub fn zebra_print_view() -> Svg {
    let h = Handle::from_memory(PRINT);
    Svg::new(h)
        .height(Length::Fill)
        .width(Length::Shrink)
        .content_fit(ContentFit::Cover)
        .style(svg::Svg::Inverse)
}

pub fn zebra_logo_view() -> Svg {
    let h = Handle::from_memory(ZEBRA_LOGO);
    Svg::new(h)
        .height(Length::Fill)
        .width(Length::Fill)
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

pub fn zebra_hooves() -> Svg {
    let h = Handle::from_memory(HOOVES_ICON);
    Svg::new(h)
        .height(Length::Fill)
        .height(Length::Fill)
        .content_fit(ContentFit::Fill)
        .style(svg::Svg::Inverse)
}

pub fn reload_icon() -> Svg {
    let h = Handle::from_memory(RELOAD_ICON);
    Svg::new(h)
        .height(Length::Fill)
        .height(Length::Fill)
        .content_fit(ContentFit::Cover)
        .style(svg::Svg::Primary)
}

pub fn copy_icon() -> Svg {
    let h = Handle::from_memory(COPY_ICON);
    Svg::new(h)
        .height(Length::Fill)
        .height(Length::Fill)
        .content_fit(ContentFit::Cover)
        .style(svg::Svg::Primary)
}

pub fn rust_logo() -> Svg {
    let h = Handle::from_memory(RUST_ICON);
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

pub fn lock_icon() -> Svg {
    let h = Handle::from_memory(LOCK_ICON);
    Svg::new(h)
        .content_fit(ContentFit::Cover)
        .style(svg::Svg::Inverse)
}

pub fn back_icon() -> Svg {
    let h = Handle::from_memory(BACK_ICON);
    Svg::new(h)
        .height(Length::Fill)
        .height(Length::Fill)
        .content_fit(ContentFit::Cover)
        .style(svg::Svg::Primary)
}

pub fn forward_icon() -> Svg {
    let h = Handle::from_memory(FORWARD_ICON);
    Svg::new(h)
        .height(Length::Fill)
        .height(Length::Fill)
        .content_fit(ContentFit::Cover)
        .style(svg::Svg::Primary)
}
