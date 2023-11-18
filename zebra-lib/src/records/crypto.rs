//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use serde::{Deserialize, Serialize};

use super::field::Field;

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoWallet {
    pub network: Option<String>,
    pub chain: Option<String>,
    pub secret_phrase: Option<String>,
    pub private_key: Option<String>,
    pub note: String,
    pub additional: Vec<Field>,
}
