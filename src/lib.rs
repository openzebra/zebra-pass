// -- Copyright (c) 2023 Rina Khasanshin
// -- Email: hicarus@yandex.ru
// -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
extern crate rust_i18n;

use rust_i18n::i18n;

pub mod app;
pub mod bip39;
pub mod config;
pub mod core;
pub mod errors;
pub mod guard;
pub mod keychain;
pub mod records;
pub mod settings;
pub mod state;
pub mod storage;

i18n!("locales", fallback = "en");
