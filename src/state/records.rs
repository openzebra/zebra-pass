//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use serde::{Deserialize, Serialize};

use super::address::PostalAddress;
use super::card::Card;
use super::crypto::CryptoWallet;
use super::identity::Identity;
use super::login::Login;
use super::swift::Swift;

#[derive(Debug, Serialize, Deserialize)]
pub enum Records {
    Login(Login),
    Card(Card),
    Swift(Swift),
    CryptoWallet(CryptoWallet),
    Identity(Identity),
    Address(PostalAddress),
}
