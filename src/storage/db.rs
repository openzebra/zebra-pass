//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use crate::storage::errors::StorageErrors;
use directories::ProjectDirs;
use sled::{Db, Error, IVec};

pub struct LocalStorage {
    tree: Db,
}

impl LocalStorage {
    pub fn new() -> Result<Self, StorageErrors> {
        let path = match ProjectDirs::from("com.zebra", "Zebra Corp", "Zebra App") {
            Some(p) => p,
            None => return Err(StorageErrors::PathError),
        };
        let tree = match sled::open(path.data_dir()) {
            Ok(t) => t,
            Err(e) => return Err(StorageErrors::StorageAccess(e.to_string())),
        };

        Ok(LocalStorage { tree })
    }

    pub fn set(&self, key: &str, data: &[u8]) -> Result<(), Error> {
        let vec = IVec::from(data);
        self.tree.insert(key, vec)?;

        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<IVec>, Error> {
        let some_value = self.tree.get(key)?;

        Ok(some_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_storage_init() {
        LocalStorage::new().unwrap();
    }

    #[test]
    fn test_set_read() {}
}
