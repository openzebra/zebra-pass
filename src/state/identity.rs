//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use serde::{Deserialize, Serialize};

use super::field::Field;

#[derive(Debug, Serialize, Deserialize)]
pub struct Identity {
    pub name: Option<String>,
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub company: Option<String>,
    pub national_insurance_number: Option<String>,
    pub passport_number: Option<u16>,
    pub licence_number: Option<u16>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub postcode: Option<u16>,
    pub note: String,
    pub additional: Vec<Field>,
}
