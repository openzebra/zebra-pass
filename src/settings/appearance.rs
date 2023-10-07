//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Themes {
    Dark,
    Light,
    Auto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppearanceSettings {
    pub theme: Themes,
}

impl AppearanceSettings {
    pub fn new() -> Self {
        Self {
            theme: Themes::Auto,
        }
    }
}
