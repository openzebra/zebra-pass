//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::{cell::RefCell, rc::Rc};

use crate::{
    bip39::mnemonic::Mnemonic,
    config::app::{APPLICATION, ORGANIZATION, QUALIFIER},
    errors::ZebraErrors,
    guard::ZebraGuard,
    records::records::Records,
    state::state::State,
    storage::db::LocalStorage,
};

pub struct Core {
    pub data: Vec<Records>,
    pub db: Rc<LocalStorage>,
    pub guard: ZebraGuard,
    pub state: Rc<RefCell<State>>,
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
        let db = Rc::new(LocalStorage::new(qualifier, organization, application)?);
        let state = Rc::new(RefCell::new(State::from(db.clone())));
        let guard = ZebraGuard::from(state.clone());
        let data = Vec::default();

        Ok(Self {
            data,
            db,
            guard,
            state,
        })
    }

    pub fn sync(&self) -> Result<(), ZebraErrors> {
        self.state.borrow_mut().sync()
    }

    pub fn init_data(
        &mut self,
        server_sync: bool,
        email: &str,
        password: &str,
        words_salt: &str,
        m: &Mnemonic,
    ) -> Result<(), ZebraErrors> {
        self.guard.bip39_cipher_from_password::<&[Records]>(
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
}
