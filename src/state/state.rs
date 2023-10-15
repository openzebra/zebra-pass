//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use crate::{
    errors::ZebraErrors,
    settings::{appearance::AppearanceSettings, cipher::CipherSettings, settings::SettingsPayload},
    storage::{db::LocalStorage, keys::SLED_STATE_KEY},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StatePayload {
    // Email for possible send emails or server iteraction
    pub email: Option<String>,
    // Server sync, for online mode, maybe more then one device sync.
    pub server_sync: bool,
    // Possible to restore password via Zebras server
    pub restoreble: bool,
    // shasum of pubKey(Bip39) need for sync and save data on server.
    pub address: String,

    // ecrypted keys session.
    pub secure_key_store: String,
    // encrypted user data.
    pub secure_data_store: String,

    // settings.
    pub settings: SettingsPayload,
}

pub struct State<'a> {
    pub payload: StatePayload,
    pub ready: bool,
    db: &'a LocalStorage,
}

impl<'a> State<'a> {
    pub fn from(db: &'a LocalStorage) -> Self {
        let appearance = AppearanceSettings::new();
        let cipher = CipherSettings::new();
        let settings = SettingsPayload { cipher, appearance };
        let payload = StatePayload {
            settings,
            email: None,
            server_sync: false,
            restoreble: false,
            address: String::default(),
            secure_key_store: String::default(),
            secure_data_store: String::default(),
        };
        let ready = false;

        Self { db, payload, ready }
    }

    pub fn update(&self) -> Result<(), ZebraErrors> {
        if !self.ready {
            return Err(ZebraErrors::StateNotRead);
        }

        self.db
            .set::<&StatePayload>(SLED_STATE_KEY, &self.payload)?;

        Ok(())
    }

    pub fn sync(&mut self) -> Result<(), ZebraErrors> {
        match self.db.get::<StatePayload>(SLED_STATE_KEY) {
            Ok(payload_store) => {
                self.payload = payload_store;
            }
            Err(_) => {
                self.db
                    .set::<&StatePayload>(SLED_STATE_KEY, &self.payload)?;
            }
        };
        self.ready = true;

        Ok(())
    }
}

#[cfg(test)]
mod settings_tests {
    use super::*;

    #[test]
    fn test_zebra_state() {
        let db = LocalStorage::new("com.test_state", "test-state Corp", "test_state App").unwrap();
        let mut state = State::from(&db);

        state.sync().unwrap();

        state.payload.settings.cipher.difficulty = 123;

        state.update().unwrap();

        let mut new_state = State::from(&db);

        new_state.sync().unwrap();

        assert_eq!(
            state.payload.settings.cipher.difficulty,
            new_state.payload.settings.cipher.difficulty
        );
    }
}
