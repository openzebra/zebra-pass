//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

pub mod appearance;
pub mod cipher;
pub mod language;

use language::Language;
use serde::{Deserialize, Serialize};

use appearance::AppearanceSettings;
use cipher::CipherSettings;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettingsPayload {
    pub appearance: AppearanceSettings,
    pub cipher: CipherSettings,
    pub locale: Language,
}
