//! -- Copyright (c) 2024 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

#[derive(Debug)]
pub enum Categories {
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
            Categories::CryptoWallet => "secure_note",
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
