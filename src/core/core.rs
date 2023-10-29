//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::{cell::RefCell, rc::Rc};

slint::include_modules!();

use serde::ser::SerializeStruct;

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
    pub db: Rc<LocalStorage>,
    pub guard: ZebraGuard,
    pub state: Rc<RefCell<State>>,
}

impl serde::Serialize for ElementItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ElementItem", 4)?;

        state.serialize_field("title", &self.title.to_string())?;
        state.serialize_field("value", &self.value.to_string())?;
        state.serialize_field("hide", &self.hide)?;
        state.serialize_field("copy", &self.copy)?;

        state.end()
    }
}

impl serde::Serialize for Element {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // fields: [ElementItem],
        // extra_fields: [ElementItem]
        let mut state = serializer.serialize_struct("Element", 9)?;

        state.serialize_field("icon", &self.icon.path())?;
        state.serialize_field("name", &self.name.to_string())?;
        state.serialize_field("website", &self.website.to_string())?;
        state.serialize_field("type", &self.r#type)?;
        state.serialize_field("created", &self.created.to_string())?;
        state.serialize_field("updated", &self.updated.to_string())?;
        state.serialize_field("favourite", &self.favourite)?;
        state.serialize_field("favourite", &self.favourite)?;

        state.end()
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
        let db = Rc::new(LocalStorage::new(qualifier, organization, application)?);
        let state = Rc::new(RefCell::new(State::from(db.clone())));
        let guard = ZebraGuard::from(state.clone());

        Ok(Self { db, guard, state })
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
            &[],
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

    pub fn records_update(&mut self, records: Vec<Element>) -> Result<(), ZebraErrors> {
        self.guard.update(records)?;

        Ok(())
    }
}

#[cfg(test)]
mod core_tests {
    use crate::bip39::mnemonic::Language;

    use super::*;
    use rand;

    #[test]
    fn test_init() {
        let mut core_data = Core::from("tes0", "tes1", "test2").unwrap();
        core_data.sync().unwrap();

        let mut rng = rand::thread_rng();
        let m = Mnemonic::gen(&mut rng, 15, Language::English).unwrap();
        let password = "password";

        core_data.init_data(false, "", &password, "", &m).unwrap();

        drop(core_data);
        drop(m);

        let mut new_core_data = Core::from("tes0", "tes1", "test2").unwrap();
        new_core_data.sync().unwrap();

        assert!(new_core_data
            .guard
            .try_unlock("invalid password".as_bytes())
            .is_err());

        assert!(new_core_data.guard.try_unlock(password.as_bytes()).is_ok());
    }
}
