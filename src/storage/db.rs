//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use crate::storage::errors::StorageErrors;
use directories::ProjectDirs;
use sled;
use sled::Db;

pub struct LocalStorage {
    path: ProjectDirs,
    tree: Db,
}

impl LocalStorage {
    pub fn new() -> Result<Self, StorageErrors> {
        let path = match ProjectDirs::from("com", "Zebra Corp", "Zebra App") {
            Some(p) => p,
            None => return Err(StorageErrors::PathError),
        };
        let tree = match sled::open(path.data_dir()) {
            Ok(t) => t,
            Err(e) => return Err(StorageErrors::StorageAccess(e.to_string())),
        };

        Ok(LocalStorage { path, tree })
    }
}

#[test]
fn test_dir_detect() {
    // if let Some(proj_dirs) = ProjectDirs::from("com", "Zebra Corp", "Zebra App") {
    //     let path = proj_dirs.config_dir();
    //     println!("{:?}", path);
    //     let tree = sled::open(path);
    // }
}
