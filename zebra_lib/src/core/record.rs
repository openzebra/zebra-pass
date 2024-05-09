//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Item {
    pub title: String,
    pub value: String,
    pub hide: bool,
    pub copy: bool,
    pub reload: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Element {
    pub icon: String,
    pub created: String,
    pub updated: String,
    pub favourite: bool,
    pub note: String,
    pub fields: Vec<Item>,
    pub extra_fields: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Categories {
    Login(Element),
    CryptoWallet(Element),
    CreditCard(Element),
    Identity(Element),
    BankAccount(Element),
    EmailAccount(Element),
    Passport(Element),
    DriverLicense(Element),
    WifiPassword(Element),
    Other(Element),
}

impl std::fmt::Display for Categories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {
            Categories::Login(_) => "login",
            Categories::CreditCard(_) => "credit_card",
            Categories::CryptoWallet(_) => "secure_wallet",
            Categories::Identity(_) => "identity",
            Categories::BankAccount(_) => "bank_account",
            Categories::EmailAccount(_) => "email_account",
            Categories::Passport(_) => "passport",
            Categories::DriverLicense(_) => "driver_license",
            Categories::WifiPassword(_) => "wifi_password",
            Categories::Other(_) => "other",
        };
        write!(f, "{}", text)
    }
}

impl Default for Element {
    fn default() -> Element {
        Element {
            icon: String::new(), // TODO: make it defualt icon
            created: String::new(),
            updated: String::new(),
            note: String::new(),
            favourite: false,
            fields: Vec::new(),
            extra_fields: Vec::new(),
        }
    }
}
