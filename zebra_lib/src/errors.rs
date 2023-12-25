//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use std::fmt;

#[derive(Debug, Clone)]
pub enum ZebraErrors {
    // Storage errors
    StorageAccessError,
    StoragePathError,
    StorageDataNotFound,
    StorageDataBroken,
    StorageHashsumError,
    StorageWriteError,
    StorageTimeWentBackwards,
    //Guard Errors:
    GuardIsNotEnable,
    GuardInvalidPassword,
    GuardBrokenData,

    // KeyChain errors:
    KeyChainKeysDamaged,
    KeyChainSliceError,
    KeyChainNTRURngError,
    KeyChainGenNTRUKeysError,
    KeyChainNTRUImportSKError,
    KeyChainNTRUImportPKError,
    KeychainDataIsNotHex,
    KeychainDataDecryptError,
    KeychainDataEncryptError,

    // Bip39
    Bip39BadWordCount(usize),
    Bip39UnknownWord(usize),
    Bip39InvalidChecksum,
    Bip39BadEntropyBitCount(usize),
    Bip39InvalidMnemonic,
    Bip39SliceError,
    Bip39NotIncluededWord,

    // State
    StateNotRead,
    StateNotInited,

    // Core
    CoreModelError,

    // password gen
    PassGenInvalidRng,

    SyncStateLock,

    RegexError,
}

impl fmt::Display for ZebraErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
