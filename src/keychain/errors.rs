//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

#[derive(Debug)]
pub enum KeyChainErrors {
    SliceError,
    RngError,
    GenKeysError,
    NTRUEncryptError,
    NTRUDecryptError,
}
