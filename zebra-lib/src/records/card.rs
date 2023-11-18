//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use serde::{Deserialize, Serialize};

use super::field::Field;

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub bank_name: Option<String>,
    pub card_holder: Option<String>,
    pub card_number: Option<String>,
    pub brand: Option<String>,
    pub secure_code: Option<u16>,
    pub note: String,
    pub additional: Vec<Field>,
}
