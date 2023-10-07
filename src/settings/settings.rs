//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use serde::{Deserialize, Serialize};

use crate::storage::db::LocalStorage;
use crate::storage::errors::StorageErrors;
use crate::storage::keys::SLED_SETTINGS_KEY;

use super::appearance::AppearanceSettings;
use super::cipher::CipherSettings;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsPayload {
    pub appearance: AppearanceSettings,
    pub cipher: CipherSettings,
}

pub struct Settings<'a> {
    pub payload: SettingsPayload,
    pub ready: bool,
    db: &'a LocalStorage,
}

impl<'a> Settings<'a> {
    pub fn from(db: &'a LocalStorage) -> Self {
        let appearance = AppearanceSettings::new();
        let cipher = CipherSettings::new();
        let payload = SettingsPayload { cipher, appearance };
        let ready = false;

        Self { payload, db, ready }
    }

    pub fn update(&self) -> Result<(), StorageErrors> {
        if !self.ready {
            return Err(StorageErrors::HashSumError);
        }

        self.db
            .set::<&SettingsPayload>(SLED_SETTINGS_KEY, &self.payload)?;

        Ok(())
    }

    pub fn load(&mut self) -> Result<(), StorageErrors> {
        let payload_store = self.db.get::<SettingsPayload>(SLED_SETTINGS_KEY)?;

        self.payload = payload_store;
        self.ready = true;

        Ok(())
    }
}
