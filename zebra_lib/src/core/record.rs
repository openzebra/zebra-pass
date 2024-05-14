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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Default)]
pub struct Element {
    pub icon: String,
    pub created: String,
    pub updated: String,
    pub favourite: bool,
    pub note: String,
    pub name: String,
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

impl Categories {
    pub fn update_element(&self, new_element: Element) -> Self {
        match self {
            Categories::Login(_) => Categories::Login(new_element),
            Categories::CreditCard(_) => Categories::CreditCard(new_element),
            Categories::CryptoWallet(_) => Categories::CryptoWallet(new_element),
            Categories::Identity(_) => Categories::Identity(new_element),
            Categories::BankAccount(_) => Categories::BankAccount(new_element),
            Categories::EmailAccount(_) => Categories::EmailAccount(new_element),
            Categories::Passport(_) => Categories::Passport(new_element),
            Categories::DriverLicense(_) => Categories::DriverLicense(new_element),
            Categories::WifiPassword(_) => Categories::WifiPassword(new_element),
            Categories::Other(_) => Categories::Other(new_element),
        }
    }

    pub fn get_value(&self) -> &Element {
        match self {
            Categories::Login(v) => v,
            Categories::CreditCard(v) => v,
            Categories::CryptoWallet(v) => v,
            Categories::Identity(v) => v,
            Categories::BankAccount(v) => v,
            Categories::EmailAccount(v) => v,
            Categories::Passport(v) => v,
            Categories::DriverLicense(v) => v,
            Categories::WifiPassword(v) => v,
            Categories::Other(v) => v,
        }
    }
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
