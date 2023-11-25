//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::language::Language;
use serde::{Deserialize, Serialize};

use super::appearance::AppearanceSettings;
use super::cipher::CipherSettings;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsPayload {
    pub appearance: AppearanceSettings,
    pub cipher: CipherSettings,
    pub locale: Language,
}
