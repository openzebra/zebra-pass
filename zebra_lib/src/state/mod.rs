//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use crate::{
    errors::ZebraErrors,
    settings::{
        appearance::AppearanceSettings, cipher::CipherSettings, language::Language, SettingsPayload,
    },
    storage::{db::LocalStorage, keys::SLED_STATE_KEY},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct State {
    // Email for possible send emails or server iteraction
    pub email: Option<String>,
    // Server sync, for online mode, maybe more then one device sync.
    pub server_sync: bool,
    // Possible to restore password via Zebras server
    pub restoreble: bool,
    // flag for understand first start or not
    pub inited: bool,
    // shasum of pubKey(Bip39) need for sync and save data on server.
    pub address: String,

    // ecrypted keys session.
    pub secure_key_store: String,
    // encrypted user data.
    pub secure_data_store: String,

    // settings.
    pub settings: SettingsPayload,

    // if we ready to work with storage
    pub ready: bool,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl State {
    pub fn new() -> Self {
        let appearance = AppearanceSettings::new();
        let cipher = CipherSettings::new();
        let locale = Language::English;
        let settings = SettingsPayload {
            cipher,
            appearance,
            locale,
        };
        State {
            settings,
            email: None,
            server_sync: false,
            restoreble: false,
            inited: false,
            address: String::default(),
            secure_key_store: String::default(),
            secure_data_store: String::default(),
            ready: false,
        }
    }

    pub fn state_update(&self, db: &LocalStorage) -> Result<(), ZebraErrors> {
        // TODO: here will be options for sync with server!
        if !self.ready {
            return Err(ZebraErrors::StateNotRead);
        }

        db.set::<&Self>(SLED_STATE_KEY, self)?;

        Ok(())
    }

    pub fn sync(&mut self, db: &LocalStorage) -> Result<(), ZebraErrors> {
        match db.get::<Self>(SLED_STATE_KEY) {
            Ok(payload_store) => {
                *self = payload_store;
            }
            Err(_) => {
                db.set::<&Self>(SLED_STATE_KEY, self)?;
            }
        };
        self.ready = true;

        Ok(())
    }
}

#[cfg(test)]
mod settings_tests {
    use crate::storage::db::LocalStorage;

    use super::*;

    #[test]
    fn test_zebra_state() {
        let db = LocalStorage::new("com.test_state", "test-state Corp", "test_state App").unwrap();

        let mut state = State::new();

        state.sync(&db).unwrap();

        state.settings.cipher.difficulty = 123;
        state.secure_key_store = String::from("test keys");
        state.secure_data_store = String::from("test data");

        state.state_update(&db).unwrap();

        let mut new_state = State::new();

        new_state.sync(&db).unwrap();

        assert_eq!(
            state.settings.cipher.difficulty,
            new_state.settings.cipher.difficulty
        );
        assert_eq!(state.secure_data_store, new_state.secure_data_store);
        assert_eq!(state.secure_key_store, new_state.secure_key_store);
    }
}
