//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

extern crate hex;
extern crate serde_json;

use crate::{
    errors::ZebraErrors,
    keychain::keys::{KeyChain, AES_KEY_SIZE},
    settings::settings::SettingsPayload,
};
use ntrulp::params::params1277::{PUBLICKEYS_BYTES, SECRETKEYS_BYTES};
use serde::{Deserialize, Serialize};

// TODO: posible to remake RC or ARC if need
// TODO: add time before lock session.
pub struct ZebraGuard<'a, 'b> {
    settings: &'a SettingsPayload,
    keys: Option<KeyChain>,
    secure_key_store: &'b str,
    secure_data_store: &'b str,
}

impl<'a, 'b> ZebraGuard<'a, 'b> {
    pub fn from(
        settings: &'a SettingsPayload,
        secure_key_store: &'b str,
        secure_data_store: &'b str,
    ) -> Self {
        Self {
            secure_data_store,
            secure_key_store,
            keys: None,
            settings,
        }
    }

    // gen_keys from password
    // -> decrypt keys_session(bip39)
    // -> decrypt secure_data via (bip39) keys
    pub fn try_unlock(&mut self, password: &[u8]) -> Result<(), ZebraErrors> {
        let orders = &self.settings.cipher.cipher_orders;
        let difficulty = self.settings.cipher.difficulty;

        let pass_keys = KeyChain::from_pass(&password, difficulty)
            .or(Err(ZebraErrors::GuardInvalidPassword))?;
        let session = pass_keys.decrypt(&self.secure_key_store, &orders)?;
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

        Ok(())
    }

    pub fn get_data<T>(&self) -> Result<T, ZebraErrors>
    where
        T: for<'c> Deserialize<'c> + Serialize,
    {
        let orders = &self.settings.cipher.cipher_orders;
        let keys = self.keys.as_ref().ok_or(ZebraErrors::GuardIsNotEnable)?;
        let json_bytes = keys.decrypt(&self.secure_data_store, &orders)?;
        let data: T =
            serde_json::from_slice(&json_bytes).or(Err(ZebraErrors::KeyChainKeysDamaged))?;

        Ok(data)
    }

    pub fn bip39_cipher_from_password<T>(
        &mut self,
        password: &[u8],
        words: &str,
        words_password: &str,
        data: T,
    ) -> Result<(String, String), ZebraErrors>
    where
        T: Serialize,
    {
        let orders = &self.settings.cipher.cipher_orders;
        let difficulty = self.settings.cipher.difficulty;
        let pwd_keys = KeyChain::from_pass(password, difficulty)?;
        let bip39_keys = KeyChain::from_bip39(words, words_password)?;
        let bip39_keys_bytes = bip39_keys.as_bytes().to_vec();
        let keys_cipher = pwd_keys.encrypt(bip39_keys_bytes, &orders)?;

        let json = serde_json::to_string(&data).or(Err(ZebraErrors::GuardBrokenData))?;
        let data_cipher = bip39_keys.encrypt(json.as_bytes().to_vec(), &orders)?;

        self.keys = Some(bip39_keys);

        Ok((data_cipher, keys_cipher))
    }

    pub fn update<T>(&mut self, data: T) -> Result<String, ZebraErrors>
    where
        T: Serialize,
    {
        let orders = &self.settings.cipher.cipher_orders;
        let bip39_keys = self.keys.as_ref().ok_or(ZebraErrors::GuardIsNotEnable)?;
        let json = serde_json::to_string(&data).or(Err(ZebraErrors::GuardBrokenData))?;
        let data_cipher = bip39_keys.encrypt(json.as_bytes().to_vec(), &orders)?;

        Ok(data_cipher)
    }
}

#[cfg(test)]
mod guard_tests {
    use rand::RngCore;

    use super::*;
    use crate::bip39::mnemonic::Mnemonic;

    // #[test]
    // fn test_init_unlock() {
    //     let mut rng = rand::thread_rng();
    //
    //     let m = Mnemonic::generate_mnemonic(&mut rng).unwrap();
    //     let db = LocalStorage::new("com.test_guard", "testGuard Corp", "TestGuard App").unwrap();
    //     let settings = Settings::from(&db);
    //     let mut guard = ZebraGuard::from(&db, &settings);
    //
    //     let mut password = [0u8; 1245];
    //     let words = m.get();
    //     let words_password = "test";
    //     let data = vec![
    //         "test0".to_string(),
    //         "test1".to_string(),
    //         "test2".to_string(),
    //     ];
    //
    //     rng.fill_bytes(&mut password);
    //
    //     assert!(!guard.enable);
    //     assert!(!guard.ready);
    //
    //     guard
    //         .init_bip39::<Vec<String>>(&password, &words, words_password, data.clone())
    //         .unwrap();
    //
    //     assert!(guard.enable);
    //     assert!(guard.ready);
    //
    //     let mut new_guard = ZebraGuard::from(&db, &settings);
    //
    //     assert!(!new_guard.enable);
    //     assert!(!new_guard.ready);
    //
    //     new_guard.load().unwrap();
    //
    //     assert!(new_guard.ready);
    //
    //     new_guard.try_unlock(&password).unwrap();
    //
    //     assert!(new_guard.enable);
    //     assert!(new_guard.ready);
    //
    //     let decrypted_data = new_guard.get_data::<Vec<String>>().unwrap();
    //
    //     assert_eq!(
    //         new_guard.keys.as_ref().unwrap().as_bytes(),
    //         guard.keys.as_ref().unwrap().as_bytes()
    //     );
    //     assert_eq!(
    //         new_guard.secure_data_store.content,
    //         guard.secure_data_store.content
    //     );
    //     assert_eq!(&data, &decrypted_data);
    //
    //     let new_data: Vec<u8> = vec![0, 1, 2, 3, 4, 5];
    //
    //     guard.update::<&[u8]>(&new_data).unwrap();
    //     new_guard.load().unwrap();
    //
    //     assert_eq!(
    //         new_guard.secure_data_store.content,
    //         guard.secure_data_store.content
    //     );
    //     assert_eq!(&data, &decrypted_data);
    // }
}
