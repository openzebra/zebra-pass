//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::borrow::Cow;

use crate::{
    keychain::keys::KeyChain,
    storage::{
        db::LocalStorage,
        errors::StorageErrors,
        keys::{SLED_DATA_KEY, SLED_KEYS_KEY},
    },
};

// TODO: posible to remake RC or ARC if need
pub struct ZebraGuard<'a> {
    // unlock state
    pub enable: bool,
    // has data from storage.
    pub ready: bool,
    db: &'a LocalStorage,
    keys: &'a KeyChain,
    secure_key_store: Cow<'a, str>,
    secure_data_store: Cow<'a, str>,
}

impl<'a> ZebraGuard<'a> {
    pub fn from(db: &'a LocalStorage, keys: &'a KeyChain) -> Self {
        let enable = false;
        let ready = false;
        let secure_key_store: Cow<'a, str> = Cow::from(String::default());
        let secure_data_store: Cow<'a, str> = Cow::from(String::default());

        Self {
            enable,
            ready,
            secure_key_store,
            secure_data_store,
            db,
            keys,
        }
    }

    pub fn sync(&mut self) -> Result<(), StorageErrors> {
        let secure_key_store = self.db.get::<String>(SLED_KEYS_KEY)?;
        let secure_data_store = self.db.get::<String>(SLED_DATA_KEY)?;

        self.secure_key_store = Cow::from(secure_key_store);
        self.secure_data_store = Cow::from(secure_data_store);
        self.ready = true;

        Ok(())
    }
}
