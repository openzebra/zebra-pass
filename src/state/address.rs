//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use serde::{Deserialize, Serialize};

use super::field::Field;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostalAddress {
    pub street: String,
    pub city: String,
    pub district: Option<String>,
    pub region: String,
    pub postal_code: String,
    pub country: String,
}
