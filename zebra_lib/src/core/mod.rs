//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
pub mod email;
pub mod passgen;
pub mod password_strength;
pub mod record;

use crate::core::record::Categories;
use crate::keychain::keys::{KeyChain, AES_KEY_SIZE};
use crate::{
    bip39::mnemonic::Mnemonic,
    config::app::{APPLICATION, ORGANIZATION, QUALIFIER},
    errors::ZebraErrors,
    state::State,
    storage::db::LocalStorage,
};
use ntrulp::params::params1277::{PUBLICKEYS_BYTES, SECRETKEYS_BYTES};
use std::{borrow::Cow, fmt, path::Path};

pub struct Core {
    pub state: State<'static>,
    pub data: Vec<Categories>,
    keys: Option<KeyChain>,
    db: LocalStorage,
}

impl fmt::Debug for Core {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.state)
            .field(&self.data)
            .finish()
    }
}

impl Core {
    pub fn new() -> Result<Self, ZebraErrors> {
        Core::from(QUALIFIER, ORGANIZATION, APPLICATION)
    }

    pub fn from(
        qualifier: &str,
        organization: &str,
        application: &str,
    ) -> Result<Self, ZebraErrors> {
        let db = LocalStorage::new(qualifier, organization, application)?;
        let state = State::new();
        let data = Vec::default();
        let keys = None;

        Ok(Self {
            db,
            state,
            data,
            keys,
        })
    }

    pub fn get_data_dir(&self) -> &Path {
        self.db.get_path()
    }

    pub fn get_data_size(&self) -> u64 {
        self.db.get_db_size()
    }

    pub fn get_storage_version(&self) -> u16 {
        self.state.version
    }

    pub fn export_to_file(&self, path: &Path) -> Result<(), ZebraErrors> {
        self.db.save_as_file(path)?;

        Ok(())
    }

    pub fn sync(&mut self) -> Result<(), ZebraErrors> {
        self.state.sync(&self.db)?;

        Ok(())
    }

    pub fn state_update(&self) -> Result<(), ZebraErrors> {
        self.state.state_update(&self.db)
    }

    pub fn data_update(&mut self) -> Result<(), ZebraErrors> {
        self.update()?;

        Ok(())
    }

    pub fn init_data(
        &mut self,
        server_sync: bool,
        email: &str,
        password: &str,
        words_salt: &str,
        m: &Mnemonic,
    ) -> Result<(), ZebraErrors> {
        self.bip39_cipher_from_password(password.as_bytes(), m, words_salt)?;

        let restoreble = !email.is_empty();

        if restoreble {
            self.state.email = Some(Cow::from(email.to_owned()));
        }

        self.state.restoreble = restoreble;
        self.state.server_sync = server_sync;
        self.state.address = self.get_address()?;
        self.state.inited = true;

        Ok(())
    }

    pub fn unlock(&mut self, password: &str) -> Result<(), ZebraErrors> {
        self.try_unlock(password.as_bytes())?;
        self.data = self.get_data()?;

        Ok(())
    }

    pub fn add_element(&mut self, elem: Categories) -> Result<(), ZebraErrors> {
        self.data.push(elem);
        self.update()?;

        // TODO: add email validator.
        // TODO: add created, updated time.

        Ok(())
    }

    pub fn remove_element(&mut self, index: usize) -> Result<(), ZebraErrors> {
        self.data.remove(index);
        self.update()?;

        Ok(())
    }

    pub fn is_unlock(&self) -> bool {
        self.keys.is_some()
    }

    // gen_keys from password
    // -> decrypt keys_session(bip39)
    // -> decrypt secure_data via (bip39) keys
    fn try_unlock(&mut self, password: &[u8]) -> Result<(), ZebraErrors> {
        let orders = &self.state.settings.cipher.cipher_orders;
        let difficulty = self.state.settings.cipher.difficulty;
        let secure_key_store = &self.state.secure_key_store;

        if !self.state.inited {
            return Err(ZebraErrors::StateNotInited);
        }
        if !self.state.ready {
            return Err(ZebraErrors::StateNotRead);
        }

        let pass_keys =
            KeyChain::from_pass(password, difficulty).or(Err(ZebraErrors::GuardInvalidPassword))?;
        let session = pass_keys.decrypt(secure_key_store, orders)?;
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

    fn get_data(&self) -> Result<Vec<Categories>, ZebraErrors> {
        let orders = &self.state.settings.cipher.cipher_orders;
        let secure_data_store = &self.state.secure_data_store;

        let keys = self.keys.as_ref().ok_or(ZebraErrors::GuardIsNotEnable)?;
        let json_bytes = keys.decrypt(secure_data_store, orders)?;

        let data = serde_json::from_slice(&json_bytes).or(Err(ZebraErrors::StorageDataBroken))?;

        Ok(data)
    }

    fn bip39_cipher_from_password(
        &mut self,
        password: &[u8],
        m: &Mnemonic,
        words_password: &str,
    ) -> Result<(), ZebraErrors> {
        let orders = &self.state.settings.cipher.cipher_orders;
        let difficulty = self.state.settings.cipher.difficulty;

        let pwd_keys = KeyChain::from_pass(password, difficulty)?;
        let bip39_keys = KeyChain::from_bip39(m, words_password)?;
        let bip39_keys_bytes = bip39_keys.as_bytes().to_vec();
        let keys_cipher = pwd_keys.encrypt(bip39_keys_bytes, orders)?;

        let json = serde_json::to_string(&self.data).or(Err(ZebraErrors::GuardBrokenData))?;
        let data_cipher = bip39_keys.encrypt(json.as_bytes().to_vec(), orders)?;

        self.keys = Some(bip39_keys);
        self.state.secure_data_store = Cow::from(data_cipher);
        self.state.secure_key_store = Cow::from(keys_cipher);
        self.state.address = self.get_address()?;
        self.state.inited = true;
        self.state.state_update(&self.db)?;

        Ok(())
    }

    fn update(&mut self) -> Result<(), ZebraErrors> {
        let orders = &self.state.settings.cipher.cipher_orders;

        let bip39_keys = self.keys.as_ref().ok_or(ZebraErrors::GuardIsNotEnable)?;
        let json = serde_json::to_string(&self.data).or(Err(ZebraErrors::GuardBrokenData))?;
        let data_cipher = bip39_keys.encrypt(json.as_bytes().to_vec(), orders)?;

        self.state.secure_data_store = Cow::from(data_cipher);
        self.state_update()?;

        Ok(())
    }

    fn get_address<'a>(&self) -> Result<Cow<'a, str>, ZebraErrors> {
        let keys = self.keys.as_ref().ok_or(ZebraErrors::GuardIsNotEnable)?;
        let hex = Cow::from(hex::encode(keys.get_address()));

        Ok(hex)
    }
}

#[cfg(test)]
mod core_tests {
    use crate::bip39::mnemonic::Language;

    use super::*;
    use crate::core::record::{Categories, Element};
    use rand;
    use rand::RngCore;

    use crate::bip39::mnemonic::Mnemonic;

    #[test]
    fn test_init() {
        let mut core_data: Core = Core::from("tes0", "tes1", "test2").unwrap();
        core_data.sync().unwrap();

        let mut rng = rand::thread_rng();
        let m = Mnemonic::gen(&mut rng, 15, Language::English).unwrap();
        let password = "password";

        core_data.init_data(false, "", password, "", &m).unwrap();

        drop(core_data);

        let mut new_core_data: Core = Core::from("tes0", "tes1", "test2").unwrap();
        new_core_data.sync().unwrap();

        assert!(new_core_data
            .try_unlock("invalid password".as_bytes())
            .is_err());

        assert!(new_core_data.try_unlock(password.as_bytes()).is_ok());
    }

    #[test]
    fn test_init_unlock() {
        let mut rng = rand::thread_rng();

        let m = Mnemonic::gen(&mut rng, 12, Language::English).unwrap();
        let mut core: Core = Core::from("tes03242", "tes12323", "test299").unwrap();

        let mut password = [0u8; 1245];
        let words_password = "test";
        let data = vec![Categories::Login(Element {
            icon: "test_icon_url".to_string(),
            name: String::new(),
            created: 0,
            updated: 0,
            note: String::new(),
            favourite: false,
            fields: vec![],
            extra_fields: vec![],
        })];

        rng.fill_bytes(&mut password);

        assert!(core.keys.is_none());
        assert!(!core.state.ready);

        assert!(core.try_unlock(&password).is_err());

        // testing init
        core.sync().unwrap();
        core.data = data.clone();
        core.bip39_cipher_from_password(&password, &m, words_password)
            .unwrap();

        assert!(core.keys.is_some());
        assert!(core.state.ready);

        let secure_data_store = core.state.secure_data_store.clone();
        let core_keys = core.keys.clone();
        drop(core);

        // testing unlock
        let mut new_core: Core = Core::from("tes03242", "tes12323", "test299").unwrap();

        assert!(new_core.keys.is_none());
        assert!(!new_core.state.ready);

        new_core.sync().unwrap();

        assert!(new_core.state.ready);

        new_core.try_unlock(&password).unwrap();

        assert!(new_core.keys.is_some());
        assert!(new_core.state.inited);

        let decrypted_data = new_core.get_data().unwrap();

        assert_eq!(
            new_core.keys.as_ref().unwrap().as_bytes(),
            core_keys.as_ref().unwrap().as_bytes()
        );
        assert_eq!(secure_data_store, new_core.state.secure_data_store);
        assert_eq!(&data, &decrypted_data);
    }
}
