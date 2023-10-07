//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use crate::keychain::keys::CipherOrders;

pub struct CipherSettings {
    pub difficulty: u32,
    pub cipher_orders: Vec<CipherOrders>,
}
