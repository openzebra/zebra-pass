//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

extern crate hex;
extern crate serde_json;

use std::{cell::RefCell, rc::Rc};

use crate::{
    bip39::mnemonic::Mnemonic,
    errors::ZebraErrors,
    keychain::keys::{KeyChain, AES_KEY_SIZE},
    state::state::State,
};
use ntrulp::params::params1277::{PUBLICKEYS_BYTES, SECRETKEYS_BYTES};
use serde::{Deserialize, Serialize};

// TODO: add time before lock session.
pub struct ZebraGuard {
    keys: Option<KeyChain>,
    state: Rc<RefCell<State>>,
}

impl ZebraGuard {
    pub fn from(state: Rc<RefCell<State>>) -> Self {
        Self { state, keys: None }
    }

    // gen_keys from password
    // -> decrypt keys_session(bip39)
    // -> decrypt secure_data via (bip39) keys
    pub fn try_unlock(&mut self, password: &[u8]) -> Result<(), ZebraErrors> {
        let state = &self.state.borrow();
        let orders = &state.payload.settings.cipher.cipher_orders;
        let difficulty = state.payload.settings.cipher.difficulty;
        let secure_key_store = &state.payload.secure_key_store;

        if !state.payload.inited {
            return Err(ZebraErrors::StateNotInited);
        }
        if !state.ready {
            return Err(ZebraErrors::StateNotRead);
        }

        let pass_keys = KeyChain::from_pass(&password, difficulty)
            .or(Err(ZebraErrors::GuardInvalidPassword))?;
        let session = pass_keys.decrypt(&secure_key_store, &orders)?;
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
        let state = &self.state.borrow().payload;
        let orders = &state.settings.cipher.cipher_orders;
        let secure_data_store = &state.secure_data_store;

        let keys = self.keys.as_ref().ok_or(ZebraErrors::GuardIsNotEnable)?;
        let json_bytes = keys.decrypt(&secure_data_store, &orders)?;
        let data: T =
            serde_json::from_slice(&json_bytes).or(Err(ZebraErrors::KeyChainKeysDamaged))?;

        Ok(data)
    }

    pub fn bip39_cipher_from_password<T>(
        &mut self,
        password: &[u8],
        m: &Mnemonic,
        words_password: &str,
        data: T,
    ) -> Result<(), ZebraErrors>
    where
        T: Serialize,
    {
        let state = &mut self.state.borrow_mut();
        let orders = &state.payload.settings.cipher.cipher_orders;
        let difficulty = state.payload.settings.cipher.difficulty;

        let pwd_keys = KeyChain::from_pass(password, difficulty)?;
        let bip39_keys = KeyChain::from_bip39(m, words_password)?;
        let bip39_keys_bytes = bip39_keys.as_bytes().to_vec();
        let keys_cipher = pwd_keys.encrypt(bip39_keys_bytes, &orders)?;

        let json = serde_json::to_string(&data).or(Err(ZebraErrors::GuardBrokenData))?;
        let data_cipher = bip39_keys.encrypt(json.as_bytes().to_vec(), &orders)?;

        self.keys = Some(bip39_keys);
        state.payload.secure_data_store = data_cipher;
        state.payload.secure_key_store = keys_cipher;
        state.payload.address = self.get_address()?;
        state.payload.inited = true;
        state.update()?;

        Ok(())
    }

    pub fn update<T>(&mut self, data: T) -> Result<(), ZebraErrors>
    where
        T: Serialize,
    {
        let state = &mut self.state.borrow_mut();
        let orders = &state.payload.settings.cipher.cipher_orders;

        let bip39_keys = self.keys.as_ref().ok_or(ZebraErrors::GuardIsNotEnable)?;
        let json = serde_json::to_string(&data).or(Err(ZebraErrors::GuardBrokenData))?;
        let data_cipher = bip39_keys.encrypt(json.as_bytes().to_vec(), &orders)?;

        state.payload.secure_data_store = data_cipher;
        state.update()?;

        Ok(())
    }

    pub fn get_address(&self) -> Result<String, ZebraErrors> {
        let keys = self.keys.as_ref().ok_or(ZebraErrors::GuardIsNotEnable)?;

        Ok(hex::encode(keys.get_address()))
    }
}

#[cfg(test)]
mod guard_tests {
    use rand::RngCore;

    use super::*;
    use crate::{
        bip39::mnemonic::{Language, Mnemonic},
        storage::db::LocalStorage,
    };

    #[test]
    fn test_init_unlock() {
        let mut rng = rand::thread_rng();

        let m = Mnemonic::gen(&mut rng, 12, Language::English).unwrap();
        let db = Rc::new(
            LocalStorage::new("com.guardtest", "TestGuard Corp", "TesingGuard App").unwrap(),
        );
        let state = Rc::new(RefCell::new(State::from(db.clone())));
        let mut guard = ZebraGuard::from(state.clone());

        let mut password = [0u8; 1245];
        let words_password = "test";
        let data = vec![
            "test0".to_string(),
            "test1".to_string(),
            "test2".to_string(),
        ];

        rng.fill_bytes(&mut password);

        assert!(guard.keys.is_none());
        assert!(!state.borrow().ready);

        assert!(guard.try_unlock(&password).is_err());

        // testing init
        state.borrow_mut().sync().unwrap();
        guard
            .bip39_cipher_from_password::<Vec<String>>(&password, &m, words_password, data.clone())
            .unwrap();

        assert!(guard.keys.is_some());
        assert!(state.borrow().ready);

        // testing unlock
        let new_state = Rc::new(RefCell::new(State::from(db.clone())));
        let mut new_guard = ZebraGuard::from(new_state.clone());

        assert!(new_guard.keys.is_none());
        assert!(!new_state.borrow().ready);

        new_state.borrow_mut().sync().unwrap();

        assert!(new_state.borrow().ready);

        new_guard.try_unlock(&password).unwrap();

        assert!(new_guard.keys.is_some());

        let decrypted_data = new_guard.get_data::<Vec<String>>().unwrap();

        assert_eq!(
            new_guard.keys.as_ref().unwrap().as_bytes(),
            guard.keys.as_ref().unwrap().as_bytes()
        );
        assert_eq!(
            state.borrow().payload.secure_data_store,
            new_state.borrow().payload.secure_data_store
        );
        assert_eq!(&data, &decrypted_data);

        let new_data: Vec<u8> = vec![0, 1, 2, 3, 4, 5];

        guard.update::<&[u8]>(&new_data).unwrap();

        assert_ne!(
            state.borrow().payload.secure_data_store,
            new_state.borrow().payload.secure_data_store
        );

        assert_eq!(&data, &decrypted_data);

        new_state.borrow_mut().sync().unwrap();

        assert_eq!(
            state.borrow().payload.secure_data_store,
            new_state.borrow().payload.secure_data_store
        );
    }
}
