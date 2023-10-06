//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

extern crate hex;
extern crate serde_json;

use crate::{
    keychain::keys::{CipherOrders, KeyChain, SecureData, AES_KEY_SIZE},
    storage::{
        db::LocalStorage,
        errors::StorageErrors,
        keys::{SLED_DATA_KEY, SLED_KEYS_KEY},
    },
};
use ntrulp::params::params1277::{PUBLICKEYS_BYTES, SECRETKEYS_BYTES};
use serde::{Deserialize, Serialize};

pub enum ZebraGuardErrors {
    IncorrectPassword,
    InvalidPassword,
    IncorrectBip39Keys,
    GuardIsNotReady,
    GuardIsNotEnable,
    KeysDamaged,
    DataDamaged,
}

// TODO: posible to remake RC or ARC if need
// TODO: add time before lock session.
pub struct ZebraGuard<'a> {
    // unlock state
    pub enable: bool,
    // has data from storage.
    pub ready: bool,
    db: &'a LocalStorage,
    keys: Option<KeyChain>,
    secure_key_store: SecureData,
    secure_data_store: SecureData,
}

impl<'a> ZebraGuard<'a> {
    pub fn from(db: &'a LocalStorage) -> Self {
        let enable = false;
        let ready = false;

        let secure_key_store = SecureData {
            content: String::default(),
            orders: [CipherOrders::AES256, CipherOrders::NTRUP1277],
        };
        let secure_data_store = SecureData {
            content: String::default(),
            orders: [CipherOrders::AES256, CipherOrders::NTRUP1277],
        };

        Self {
            keys: None,
            enable,
            ready,
            secure_key_store,
            secure_data_store,
            db,
        }
    }

    // gen_keys from password
    // -> decrypt keys_session(bip39)
    // -> decrypt secure_data via (bip39) keys
    pub fn try_unlock(&mut self, password: &[u8]) -> Result<(), ZebraGuardErrors> {
        if !self.ready {
            return Err(ZebraGuardErrors::GuardIsNotReady);
        }

        let pass_keys =
            KeyChain::from_pass(&password).or(Err(ZebraGuardErrors::InvalidPassword))?;

        let session = pass_keys
            .decrypt(
                &self.secure_key_store.content,
                &self.secure_key_store.orders,
            )
            .or(Err(ZebraGuardErrors::KeysDamaged))?;
        let aes_key: [u8; AES_KEY_SIZE] = session[..AES_KEY_SIZE]
            .try_into()
            .or(Err(ZebraGuardErrors::KeysDamaged))?;
        let pq_pk: [u8; PUBLICKEYS_BYTES] = session[AES_KEY_SIZE..PUBLICKEYS_BYTES]
            .try_into()
            .or(Err(ZebraGuardErrors::KeysDamaged))?;
        let pq_sk: [u8; SECRETKEYS_BYTES] = session[AES_KEY_SIZE + PUBLICKEYS_BYTES..]
            .try_into()
            .or(Err(ZebraGuardErrors::KeysDamaged))?;
        let bip39_keys =
            KeyChain::from_keys(aes_key, pq_sk, pq_pk).or(Err(ZebraGuardErrors::KeysDamaged))?;

        self.keys = Some(bip39_keys);
        self.enable = true;

        Ok(())
    }

    pub fn get_data<ST>(&self) -> Result<ST, ZebraGuardErrors>
    where
        ST: for<'b> Deserialize<'b> + Serialize,
    {
        if !self.ready {
            return Err(ZebraGuardErrors::GuardIsNotReady);
        }
        if !self.enable {
            return Err(ZebraGuardErrors::GuardIsNotReady);
        }

        let keys = self
            .keys
            .as_ref()
            .ok_or(ZebraGuardErrors::GuardIsNotEnable)?;
        let json_bytes = keys
            .decrypt(
                &self.secure_data_store.content,
                &self.secure_data_store.orders,
            )
            .or(Err(ZebraGuardErrors::DataDamaged))?;
        let data: ST =
            serde_json::from_slice(&json_bytes).or(Err(ZebraGuardErrors::DataDamaged))?;

        Ok(data)
    }

    pub fn init_bip39(
        &mut self,
        password: &[u8],
        words: &str,
        words_password: &str,
    ) -> Result<(), ZebraGuardErrors> {
        let pwd_keys = KeyChain::from_pass(password).or(Err(ZebraGuardErrors::InvalidPassword))?;
        let bip39_keys = KeyChain::from_bip39(words, words_password)
            .or(Err(ZebraGuardErrors::IncorrectBip39Keys))?;
        let bip39_keys_bytes = bip39_keys.as_bytes().to_vec();
        let cipher = pwd_keys
            .encrypt(bip39_keys_bytes)
            .or(Err(ZebraGuardErrors::InvalidPassword))?;

        Ok(())
    }

    pub fn sync(&mut self) -> Result<(), StorageErrors> {
        let secure_key_store = self.db.get::<SecureData>(SLED_KEYS_KEY)?;
        let secure_data_store = self.db.get::<SecureData>(SLED_DATA_KEY)?;

        self.secure_key_store = secure_key_store;
        self.secure_data_store = secure_data_store;
        self.ready = true;

        Ok(())
    }
}

#[cfg(test)]
mod guard_tests {
    use super::*;
    use crate::bip39::mnemonic::Mnemonic;

    // #[test]
    // fn test_init_unlock() {
    //     let mut rng = rand::thread_rng();
    //
    //     let m = Mnemonic::generate_mnemonic(&mut rng).unwrap();
    //     let db = LocalStorage::new().unwrap();
    //     let guard = ZebraGuard::from(&db);
    // }
}
