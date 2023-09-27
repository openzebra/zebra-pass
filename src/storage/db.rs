//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use crate::storage::errors::StorageErrors;
use directories::ProjectDirs;
use sled::Db;

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

    pub fn set(&self, key: &str) {
        // self.tree.set(&"key");
    }

    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageErrors> {
        let value = match self.tree.get(key) {
            Ok(some_value) => {
                if let Some(value) = some_value {
                    value.to_vec()
                } else {
                    return Ok(None);
                }
            }
            Err(e) => return Err(StorageErrors::StorageRead(e.to_string())),
        };

        Ok(Some(value.to_vec()))
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
