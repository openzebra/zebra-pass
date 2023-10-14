//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

#[derive(Debug)]
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
    GuardIsNotReady,
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
}