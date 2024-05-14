//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Themes {
    Dark,
    Light,
    Auto,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppearanceSettings {
    pub theme: Themes,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl AppearanceSettings {
    pub fn new() -> Self {
        Self {
            theme: Themes::Auto,
        }
    }
}
