//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Default, Clone)]
pub enum Categories {
    #[default]
    Login,
    CryptoWallet,
    CreditCard,
    Identity,
    BankAccount,
    EmailAccount,
    Passport,
    DriverLicense,
    WifiPassword,
    Other,
}

impl std::fmt::Display for Categories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let text = match self {
            Categories::Login => "login",
            Categories::CreditCard => "credit_card",
            Categories::CryptoWallet => "secure_wallet",
            Categories::Identity => "identity",
            Categories::BankAccount => "bank_account",
            Categories::EmailAccount => "email_account",
            Categories::Passport => "passport",
            Categories::DriverLicense => "driver_license",
            Categories::WifiPassword => "wifi_password",
            Categories::Other => "other",
        };
        write!(f, "{}", text)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Item {
    pub title: String,
    pub value: String,
    pub hide: bool,
    pub copy: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Element {
    pub name: String,
    pub website: String,
    pub icon: String,
    pub element_type: Categories,
    pub created: String,
    pub updated: String,
    pub favourite: bool,
    pub fields: Vec<Item>,
    pub extra_fields: Vec<Item>,
}
