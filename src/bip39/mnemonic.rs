//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::borrow::Cow;

use pbkdf2::pbkdf2_hmac_array;
use sha2::{Digest, Sha256, Sha512};
use unicode_normalization::UnicodeNormalization;

use super::{errors::Bip39Error, language};

const SALT_PREFIX: &str = "zebra-bip39-mnemonic";
const MIN_NB_WORDS: usize = 12;
const EOF: u16 = u16::max_value();

#[derive(Debug)]
pub enum Mnemonic<const SIZE: usize> {
    English([u16; SIZE]),
}

fn is_invalid_word_count(word_count: usize) -> bool {
    word_count < MIN_NB_WORDS || word_count % 3 != 0
}

fn normalize_utf8_cow<'a>(cow: &mut Cow<'a, str>) {
    let is_nfkd = unicode_normalization::is_nfkd_quick(cow.as_ref().chars());
    if is_nfkd != unicode_normalization::IsNormalized::Yes {
        *cow = Cow::Owned(cow.as_ref().nfkd().to_string());
    }
}

impl<const SIZE: usize> Mnemonic<SIZE> {
    pub fn mnemonic_to_entropy(mnemonic: &str) -> Result<Self, Bip39Error> {
        // TODO: make detect lang.
        let nb_words = mnemonic.split_whitespace().count();

        if is_invalid_word_count(nb_words) {
            return Err(Bip39Error::BadWordCount(nb_words));
        }

        let mut words = [EOF; SIZE];
        let mut bits = vec![false; SIZE * 11];

        for (i, word) in mnemonic.split_whitespace().enumerate() {
            let index = language::english::WORDS
                .iter()
                .position(|w| *w == word)
                .ok_or(Bip39Error::UnknownWord(i))?;

            words[i] = index as u16;

            for j in 0..11 {
                bits[i * 11 + j] = index >> (10 - j) & 1 == 1;
            }
        }

        let mut entropy = vec![0u8; SIZE / 3 * 4];
        let nb_bytes_entropy = nb_words / 3 * 4;
        for i in 0..nb_bytes_entropy {
            for j in 0..8 {
                if bits[i * 8 + j] {
                    entropy[i] += 1 << (7 - j);
                }
            }
        }
        // TODO: replace to keccak
        let mut hasher = Sha256::new();
        hasher.update(&entropy[0..nb_bytes_entropy]);
        let check = hasher.finalize();

        for i in 0..nb_bytes_entropy / 4 {
            if bits[8 * nb_bytes_entropy + i] != ((check[i / 8] & (1 << (7 - (i % 8)))) > 0) {
                return Err(Bip39Error::InvalidChecksum);
            }
        }

        Ok(Self::English(words))
    }

    pub fn entropy_to_mnemonic(entropy: &[u8]) -> Result<Self, Bip39Error> {
        const MAX_ENTROPY_BITS: usize = 256;
        const MIN_ENTROPY_BITS: usize = 128;
        const MAX_CHECKSUM_BITS: usize = 8;

        let nb_bytes = entropy.len();
        let nb_bits = nb_bytes * 8;

        if nb_bits % 32 != 0 {
            return Err(Bip39Error::BadEntropyBitCount(nb_bits));
        }
        if nb_bits < MIN_ENTROPY_BITS || nb_bits > MAX_ENTROPY_BITS {
            return Err(Bip39Error::BadEntropyBitCount(nb_bits));
        }

        // TODO: replace to keccak
        let mut hasher = Sha256::new();
        hasher.update(&entropy);
        let check = hasher.finalize();

        let mut bits = [false; MAX_ENTROPY_BITS + MAX_CHECKSUM_BITS];
        for i in 0..nb_bytes {
            for j in 0..8 {
                bits[i * 8 + j] = (entropy[i] & (1 << (7 - j))) > 0;
            }
        }
        for i in 0..nb_bytes / 4 {
            bits[8 * nb_bytes + i] = (check[i / 8] & (1 << (7 - (i % 8)))) > 0;
        }

        let mut words = [EOF; SIZE];
        let nb_words = nb_bytes * 3 / 4;
        for i in 0..nb_words {
            let mut idx = 0;
            for j in 0..11 {
                if bits[i * 11 + j] {
                    idx += 1 << (10 - j);
                }
            }
            words[i] = idx;
        }

        Ok(Self::English(words))
    }

    pub fn get(&self) -> String {
        match self {
            Mnemonic::English(points) => points
                .iter()
                .map(|i| language::english::WORDS[*i as usize])
                .collect::<Vec<&str>>()
                .join(" "),
        }
    }
}

fn generate_mnemonic() {}

fn validate_mnemonic() {}

#[test]
fn test_mnemonic() {
    let words = "abstract silly element program name ten champion thing odor nerve wasp smooth";
    let m = Mnemonic::<12>::mnemonic_to_entropy(words).unwrap();

    assert_eq!(m.get(), words);
}
