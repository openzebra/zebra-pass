/// A BIP39 error.
#[derive(Debug)]
pub enum Bip39Error {
    BadWordCount(usize),
    UnknownWord(usize),
    BadEntropyBitCount(usize),
    InvalidChecksum,
}
