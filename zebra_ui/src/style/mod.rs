//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

pub mod application;
pub mod button;
pub mod checkbox;
pub mod container;
pub mod pick_list;
pub mod radio;
pub mod scrollable;
pub mod slider;
pub mod svg;
pub mod text;
pub mod text_input;

use crate::color::ZebraPalette;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Theme {
    Light(ZebraPalette),
    Dark(ZebraPalette),
}

impl Default for Theme {
    fn default() -> Self {
        Self::Light(Default::default())
    }
}
