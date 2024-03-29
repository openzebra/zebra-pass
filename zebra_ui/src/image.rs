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
const ADD_ICON: &[u8] = include_bytes!("../static/icons/add.svg");
const OPEN_EYE_ICON: &[u8] = include_bytes!("../static/icons/open_eye.svg");
const CLOSE_EYE_ICON: &[u8] = include_bytes!("../static/icons/close_eye.svg");
const HOOVES_ICON: &[u8] = include_bytes!("../static/icons/hooves.svg");
const RELOAD_ICON: &[u8] = include_bytes!("../static/icons/reload.svg");
const MAGIC_ICON: &[u8] = include_bytes!("../static/icons/magic.svg");
const COPY_ICON: &[u8] = include_bytes!("../static/icons/copy.svg");
const LOCK_ICON: &[u8] = include_bytes!("../static/icons/lock.svg");
const GEAR_ICON: &[u8] = include_bytes!("../static/icons/gear.svg");
const ZEBRA_LOGO: &[u8] = include_bytes!("../static/imgs/zebra_logo.svg");
const ZEBRA_BAD: &[u8] = include_bytes!("../static/icons/bad-zebra.svg");

const BITWARDEN_LOGO: &[u8] = include_bytes!("../static/icons/bitwarden.svg");

pub fn zebra_print_view() -> Svg {
    let h = Handle::from_memory(PRINT);
    Svg::new(h)
        .height(Length::Fill)
        .width(Length::Shrink)
        .content_fit(ContentFit::Cover)
        .style(svg::Svg::Inverse)
}

pub fn bad_zebra_view() -> Svg {
    let h = Handle::from_memory(ZEBRA_BAD);
    Svg::new(h).style(svg::Svg::Normal)
}

pub fn bitwarden_logo_view() -> Svg {
    let h = Handle::from_memory(BITWARDEN_LOGO);
    Svg::new(h)
        .height(Length::Fill)
        .width(Length::Fill)
        .content_fit(ContentFit::Cover)
        .style(svg::Svg::Inverse)
}

pub fn close_eye_icon() -> Svg {
    let h = Handle::from_memory(CLOSE_EYE_ICON);
    Svg::new(h)
        .height(Length::Fill)
        .width(Length::Fill)
        .content_fit(ContentFit::Cover)
        .style(svg::Svg::Primary)
}

pub fn open_eye_icon() -> Svg {
    let h = Handle::from_memory(OPEN_EYE_ICON);
    Svg::new(h)
        .height(Length::Fill)
        .width(Length::Fill)
        .content_fit(ContentFit::Cover)
        .style(svg::Svg::Primary)
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

pub fn magic_icon() -> Svg {
    let h = Handle::from_memory(MAGIC_ICON);
    Svg::new(h)
        .height(Length::Fill)
        .height(Length::Fill)
        .content_fit(ContentFit::Cover)
        .style(svg::Svg::Primary)
}

pub fn add_icon() -> Svg {
    let h = Handle::from_memory(ADD_ICON);
    Svg::new(h)
        .height(Length::Fill)
        .height(Length::Fill)
        .content_fit(ContentFit::Cover)
        .style(svg::Svg::Primary)
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

pub fn gear_icon() -> Svg {
    let h = Handle::from_memory(GEAR_ICON);
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
