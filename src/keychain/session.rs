//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use crate::storage::db::LocalStorage;

use super::keys::KeyChain;

pub struct Session {
    keys: KeyChain,
    db: LocalStorage,
}

impl Session {
    pub fn from_storage() {}
}
