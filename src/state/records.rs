//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use serde::{Deserialize, Serialize};

use super::login::Login;

#[derive(Debug, Serialize, Deserialize)]
pub enum RecordType {
    Login(Login),
    Card,
    Swift,
    CryptoWallet,
    Identity,
}
