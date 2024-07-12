//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Themes {
    Dark,
    Light,
    Auto,
}

impl ToString for Themes {
    fn to_string(&self) -> String {
        match self {
            Self::Auto => "auto".to_owned(),
            Self::Dark => "dark".to_owned(),
            Self::Light => "light".to_owned(),
        }
    }
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
