//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use super::field::Field;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub totp: Option<String>,
    pub url: Option<String>,
    pub note: String,
    pub additional: Vec<Field>,
}
