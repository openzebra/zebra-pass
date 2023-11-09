//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{
    bip39::mnemonic::Mnemonic,
    config::app::{APPLICATION, ORGANIZATION, QUALIFIER},
    errors::ZebraErrors,
    guard::ZebraGuard,
    state::state::State,
    storage::db::LocalStorage,
};

pub struct Core<T> {
    pub db: Rc<LocalStorage>,
    pub guard: ZebraGuard,
    pub state: Rc<RefCell<State>>,
    pub data: Vec<T>,
}

impl<T> Core<T>
where
    T: for<'c> Deserialize<'c> + Serialize,
{
    pub fn new() -> Result<Self, ZebraErrors> {
        Core::from(QUALIFIER, ORGANIZATION, APPLICATION)
    }

    pub fn from(
        qualifier: &str,
        organization: &str,
        application: &str,
    ) -> Result<Self, ZebraErrors> {
        let db = Rc::new(LocalStorage::new(qualifier, organization, application)?);
        let state = Rc::new(RefCell::new(State::from(db.clone())));
        let guard = ZebraGuard::from(state.clone());
        let data = Vec::default();

        Ok(Self {
            db,
            guard,
            state,
            data,
        })
    }

    pub fn sync(&self) -> Result<(), ZebraErrors> {
        self.state.borrow_mut().sync()?;

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
        self.guard.bip39_cipher_from_password::<&[T]>(
            password.as_bytes(),
            m,
            &words_salt,
            &self.data,
        )?;

        let mut state = self.state.borrow_mut();
        let restoreble = email.is_empty();

        if restoreble {
            state.payload.email = Some(email.to_string());
        }

        state.payload.restoreble = restoreble;
        state.payload.server_sync = server_sync;
        state.payload.address = self.guard.get_address()?;
        state.payload.inited = true;

        Ok(())
    }

    pub fn unlock(&mut self, password: &str) -> Result<(), ZebraErrors> {
        self.guard.try_unlock(&password.as_bytes())?;
        self.data = self.guard.get_data()?;

        Ok(())
    }

    pub fn add_element(&mut self, elem: T) -> Result<(), ZebraErrors> {
        self.data.push(elem);
        self.guard.update(&self.data)?;

        // TODO: add email validator.
        // TODO: add created, updated time.

        Ok(())
    }

    pub fn remove_element(&mut self, index: usize) -> Result<(), ZebraErrors> {
        self.data.remove(index);
        self.guard.update(&self.data)?;

        Ok(())
    }
}

#[cfg(test)]
mod core_tests {
    use crate::bip39::mnemonic::Language;

    use super::*;
    use rand;
    use serde::Deserialize;

    #[derive(Debug, Serialize, Deserialize)]
    struct TEST {}

    #[test]
    fn test_init() {
        let mut core_data: Core<TEST> = Core::from("tes0", "tes1", "test2").unwrap();
        core_data.sync().unwrap();

        let mut rng = rand::thread_rng();
        let m = Mnemonic::gen(&mut rng, 15, Language::English).unwrap();
        let password = "password";

        core_data.init_data(false, "", &password, "", &m).unwrap();

        drop(core_data);
        drop(m);

        let mut new_core_data: Core<TEST> = Core::from("tes0", "tes1", "test2").unwrap();
        new_core_data.sync().unwrap();

        assert!(new_core_data
            .guard
            .try_unlock("invalid password".as_bytes())
            .is_err());

        assert!(new_core_data.guard.try_unlock(password.as_bytes()).is_ok());
    }
}
