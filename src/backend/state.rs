//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use super::guard::ZebraGuard;
use crate::{errors::ZebraErrors, settings::settings::Settings};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StatePayload<T> {
    // Email for possible send emails or server iteraction
    pub email: Option<String>,
    // Server sync, for online mode, maybe more then one device sync.
    pub server_sync: bool,
    // Possible to restore password via Zebras server
    pub restoreble: bool,
    // shasum of pubKey(Bip39) need for sync and save data on server.
    pub address: String,
    // Storage of users data.
    pub data: T,
}

pub struct State<'a, 'b, T> {
    pub payload: Option<StatePayload<T>>,
    guard: &'a ZebraGuard<'a, 'b>,
    settings: &'b Settings<'b>,
}

impl<'a, 'b, 'c, T> State<'a, 'b, T>
where
    T: Deserialize<'c> + Serialize,
{
    pub fn from(guard: &'a ZebraGuard<'a, 'b>, settings: &'b Settings) -> Self {
        Self {
            guard,
            settings,
            payload: None,
        }
    }

    pub fn sync(&mut self) -> Result<(), ZebraErrors> {
        Ok(())
    }
}
