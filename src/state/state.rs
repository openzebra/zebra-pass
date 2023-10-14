//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use crate::settings::settings::SettingsPayload;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StatePayload {
    // Email for possible send emails or server iteraction
    pub email: Option<String>,
    // Server sync, for online mode, maybe more then one device sync.
    pub server_sync: bool,
    // Possible to restore password via Zebras server
    pub restoreble: bool,
    // shasum of pubKey(Bip39) need for sync and save data on server.
    pub address: String,

    pub secure_key_store: String,
    pub secure_data_store: String,

    pub settings: SettingsPayload,

    pub data: String,
}
