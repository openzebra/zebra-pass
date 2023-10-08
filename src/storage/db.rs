//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
extern crate hex;
extern crate serde;
extern crate serde_json;

use directories::ProjectDirs;
use sha2::{Digest, Sha256};
use sled::{Db, IVec};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::errors::ZebraErrors;

#[derive(Deserialize, Serialize)]
pub struct Data<ST> {
    pub payload: ST,
    // Storage verions
    pub version: u16,
    // last update for sync with server
    pub last_update: u64,
    // hash sum for compare with server
    pub hashsum: String,
}

pub struct LocalStorage {
    tree: Db,
    version: u16,
}

impl LocalStorage {
    /// let db = LocalStorage::new("com.zebra", "Zebra Corp", "Zebra App").unwrap();
    pub fn new(
        qualifier: &str,
        organization: &str,
        application: &str,
    ) -> Result<Self, ZebraErrors> {
        let path = match ProjectDirs::from(qualifier, organization, application) {
            Some(p) => p,
            None => return Err(ZebraErrors::StoragePathError),
        };
        let tree = match sled::open(path.data_dir()) {
            Ok(t) => t,
            Err(_) => return Err(ZebraErrors::StorageAccessError),
        };
        let version = 0;

        Ok(LocalStorage { tree, version })
    }

    pub fn get<ST>(&self, key: &str) -> Result<ST, ZebraErrors>
    where
        ST: for<'a> Deserialize<'a> + Serialize,
    {
        let some_value = self
            .tree
            .get(key)
            .or(Err(ZebraErrors::StorageAccessError))?;
        let value = some_value.ok_or(ZebraErrors::StorageDataNotFound)?;
        let json = String::from_utf8_lossy(&value);

        let data: Data<ST> = serde_json::from_str(&json).or(Err(ZebraErrors::StorageDataBroken))?;
        let json_payload =
            serde_json::to_string(&data.payload).or(Err(ZebraErrors::StorageDataBroken))?;
        let hashsum = self.hash(&json_payload.as_bytes());

        if hashsum != data.hashsum {
            return Err(ZebraErrors::StorageHashsumError);
        }

        Ok(data.payload)
    }

    pub fn set<ST>(&self, key: &str, payload: ST) -> Result<(), ZebraErrors>
    where
        ST: Serialize,
    {
        let last_update = self.get_unix_time()?;
        let json_payload =
            serde_json::to_string(&payload).or(Err(ZebraErrors::StorageDataBroken))?;
        let hashsum = self.hash(&json_payload.as_bytes());
        let data = Data {
            payload,
            hashsum,
            last_update,
            version: self.version,
        };
        let json = serde_json::to_string(&data).or(Err(ZebraErrors::StorageDataBroken))?;
        let vec = IVec::from(json.as_bytes());

        self.tree
            .insert(key, vec)
            .or(Err(ZebraErrors::StorageWriteError))?;

        Ok(())
    }

    fn hash(&self, bytes: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let hashsum = hasher.finalize();

        hex::encode(hashsum)
    }

    fn get_unix_time(&self) -> Result<u64, ZebraErrors> {
        let now = SystemTime::now();
        let since_epoch = now
            .duration_since(UNIX_EPOCH)
            .or(Err(ZebraErrors::StorageTimeWentBackwards))?;
        let u64_time = since_epoch.as_secs();

        Ok(u64_time)
    }
}

#[cfg(test)]
mod storage_tests {
    use super::*;

    #[test]
    fn test_read_write() {
        const KEY: &str = "TEST_KEY_FOR_STORAGE";

        let db = LocalStorage::new("com.test_write", "WriteTest Corp", "WriteTest App").unwrap();
        let payload = vec!["test1", "test2", "test3"];

        db.set(KEY, &payload).unwrap();

        let out = db.get::<Vec<String>>(KEY).unwrap();

        assert_eq!(out, payload);
    }
}
