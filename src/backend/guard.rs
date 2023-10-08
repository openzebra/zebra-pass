//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

extern crate hex;
extern crate serde_json;

use crate::{
    errors::ZebraErrors,
    keychain::keys::{CipherOrders, KeyChain, SecureData, AES_KEY_SIZE},
    storage::{
        db::LocalStorage,
        keys::{SLED_DATA_KEY, SLED_KEYS_KEY},
    },
};
use ntrulp::params::params1277::{PUBLICKEYS_BYTES, SECRETKEYS_BYTES};
use serde::{Deserialize, Serialize};

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
    pub fn try_unlock(&mut self, password: &[u8]) -> Result<(), ZebraErrors> {
        if !self.ready {
            return Err(ZebraErrors::GuardIsNotReady);
        }

        let pass_keys =
            KeyChain::from_pass(&password).or(Err(ZebraErrors::GuardInvalidPassword))?;

        let session = pass_keys.decrypt(
            &self.secure_key_store.content,
            &self.secure_key_store.orders,
        )?;
        let aes_key: [u8; AES_KEY_SIZE] = session[..AES_KEY_SIZE]
            .try_into()
            .or(Err(ZebraErrors::KeyChainKeysDamaged))?;
        let pq_pk: [u8; PUBLICKEYS_BYTES] = session[AES_KEY_SIZE..PUBLICKEYS_BYTES + AES_KEY_SIZE]
            .try_into()
            .or(Err(ZebraErrors::KeyChainKeysDamaged))?;
        let pq_sk: [u8; SECRETKEYS_BYTES] = session[AES_KEY_SIZE + PUBLICKEYS_BYTES..]
            .try_into()
            .or(Err(ZebraErrors::KeyChainKeysDamaged))?;
        let bip39_keys = KeyChain::from_keys(aes_key, pq_sk, pq_pk)?;

        self.keys = Some(bip39_keys);
        self.enable = true;

        Ok(())
    }

    pub fn get_data<T>(&self) -> Result<T, ZebraErrors>
    where
        T: for<'b> Deserialize<'b> + Serialize,
    {
        if !self.ready {
            return Err(ZebraErrors::GuardIsNotReady);
        }
        if !self.enable {
            return Err(ZebraErrors::GuardIsNotEnable);
        }

        let keys = self.keys.as_ref().ok_or(ZebraErrors::GuardIsNotEnable)?;
        let json_bytes = keys.decrypt(
            &self.secure_data_store.content,
            &self.secure_data_store.orders,
        )?;
        let data: T =
            serde_json::from_slice(&json_bytes).or(Err(ZebraErrors::KeyChainKeysDamaged))?;

        Ok(data)
    }

    pub fn init_bip39<T>(
        &mut self,
        password: &[u8],
        words: &str,
        words_password: &str,
        data: T,
    ) -> Result<(), ZebraErrors>
    where
        T: Serialize,
    {
        let pwd_keys = KeyChain::from_pass(password)?;
        let bip39_keys = KeyChain::from_bip39(words, words_password)?;
        let bip39_keys_bytes = bip39_keys.as_bytes().to_vec();
        let keys_cipher = pwd_keys.encrypt(bip39_keys_bytes)?;

        let json = serde_json::to_string(&data).or(Err(ZebraErrors::GuardBrokenData))?;
        let data_cipher = bip39_keys.encrypt(json.as_bytes().to_vec())?;

        self.secure_data_store = data_cipher;
        self.secure_key_store = keys_cipher;
        self.keys = Some(bip39_keys);
        self.ready = true;
        self.enable = true;

        self.db
            .set::<SecureData>(SLED_KEYS_KEY, self.secure_key_store.clone())?;
        self.db
            .set::<SecureData>(SLED_DATA_KEY, self.secure_data_store.clone())?;

        Ok(())
    }

    pub fn update<T>(&mut self, data: T) -> Result<(), ZebraErrors>
    where
        T: Serialize,
    {
        if !self.ready {
            return Err(ZebraErrors::GuardIsNotReady);
        }
        if !self.enable {
            return Err(ZebraErrors::GuardIsNotEnable);
        }

        let bip39_keys = self.keys.as_ref().ok_or(ZebraErrors::GuardIsNotEnable)?;
        let json = serde_json::to_string(&data).or(Err(ZebraErrors::GuardBrokenData))?;
        let data_cipher = bip39_keys.encrypt(json.as_bytes().to_vec())?;

        self.secure_data_store = data_cipher;
        self.db
            .set::<SecureData>(SLED_DATA_KEY, self.secure_data_store.clone())?;

        Ok(())
    }

    pub fn load(&mut self) -> Result<(), ZebraErrors> {
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
    use rand::RngCore;

    use super::*;
    use crate::bip39::mnemonic::Mnemonic;

    #[test]
    fn test_init_unlock() {
        let mut rng = rand::thread_rng();

        let m = Mnemonic::generate_mnemonic(&mut rng).unwrap();
        let db = LocalStorage::new("com.test_guard", "testGuard Corp", "TestGuard App").unwrap();
        let mut guard = ZebraGuard::from(&db);

        let mut password = [0u8; 1245];
        let words = m.get();
        let words_password = "test";
        let data = vec![
            "test0".to_string(),
            "test1".to_string(),
            "test2".to_string(),
        ];

        rng.fill_bytes(&mut password);

        assert!(!guard.enable);
        assert!(!guard.ready);

        guard
            .init_bip39::<Vec<String>>(&password, &words, words_password, data.clone())
            .unwrap();

        assert!(guard.enable);
        assert!(guard.ready);

        let mut new_guard = ZebraGuard::from(&db);

        assert!(!new_guard.enable);
        assert!(!new_guard.ready);

        new_guard.load().unwrap();

        assert!(new_guard.ready);

        new_guard.try_unlock(&password).unwrap();

        assert!(new_guard.enable);
        assert!(new_guard.ready);

        let decrypted_data = new_guard.get_data::<Vec<String>>().unwrap();

        assert_eq!(
            new_guard.keys.as_ref().unwrap().as_bytes(),
            guard.keys.as_ref().unwrap().as_bytes()
        );
        assert_eq!(
            new_guard.secure_data_store.content,
            guard.secure_data_store.content
        );
        assert_eq!(&data, &decrypted_data);

        let new_data: Vec<u8> = vec![0, 1, 2, 3, 4, 5];

        guard.update::<&[u8]>(&new_data).unwrap();
        new_guard.load().unwrap();

        assert_eq!(
            new_guard.secure_data_store.content,
            guard.secure_data_store.content
        );
        assert_eq!(&data, &decrypted_data);
    }
}
