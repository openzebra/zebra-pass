//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use super::field::Field;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Swift {
    pub iban: String,
    pub bank_name: Option<String>,
    pub bank_branch: Option<String>,
    pub swift_code: Option<String>,
    pub country: Option<String>,
    pub address: Option<String>,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub note: String,
    pub additional: Vec<Field>,
}
