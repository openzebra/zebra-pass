//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::{cell::RefCell, rc::Rc};

use crate::{
    config::app::{APPLICATION, ORGANIZATION, QUALIFIER},
    errors::ZebraErrors,
    guard::ZebraGuard,
    records::records::Records,
    state::state::State,
    storage::db::LocalStorage,
};

pub struct Core {
    pub data: Vec<Records>,
    db: Rc<LocalStorage>,
    guard: ZebraGuard,
    state: Rc<RefCell<State>>,
}

impl Core {
    pub fn new() -> Result<Self, ZebraErrors> {
        let db = Rc::new(LocalStorage::new(QUALIFIER, ORGANIZATION, APPLICATION)?);
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
}
