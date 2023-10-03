//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::borrow::Cow;

use pbkdf2::pbkdf2_hmac_array;
use sha2::{Digest, Sha256, Sha512};
use unicode_normalization::UnicodeNormalization;

use super::{config::NUMBER_WORDS, errors::Bip39Error, language};

const SALT_PREFIX: &str = "zebra-bip39-mnemonic";
const SIZE: usize = 24;
const STRENGTH: usize = 32;
const EOF: u16 = u16::max_value();

#[derive(Debug)]
pub enum Mnemonic {
    English([u16; SIZE]),
}

fn is_invalid_word_count(word_count: usize) -> bool {
    word_count < SIZE || word_count % 3 != 0
}

fn normalize_utf8_cow<'a>(cow: &mut Cow<'a, str>) {
    let is_nfkd = unicode_normalization::is_nfkd_quick(cow.as_ref().chars());
    if is_nfkd != unicode_normalization::IsNormalized::Yes {
        *cow = Cow::Owned(cow.as_ref().nfkd().to_string());
    }
}

impl Mnemonic {
    pub fn mnemonic_to_entropy(mnemonic: &str) -> Result<Self, Bip39Error> {
        // TODO: make detect lang.
        let nb_words = mnemonic.split_whitespace().count();

        if is_invalid_word_count(nb_words) {
            return Err(Bip39Error::BadWordCount(nb_words));
        }

        let mut words = [EOF; SIZE];
        let mut bits = [false; SIZE * 11];

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

        let mut entropy = [0u8; SIZE / 3 * 4];
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

    pub fn entropy_to_mnemonic(entropy: &[u8; STRENGTH]) -> Result<Self, Bip39Error> {
        const MAX_ENTROPY_BITS: usize = 256;
        const MAX_CHECKSUM_BITS: usize = 8;

        let nb_bytes = entropy.len();
        let nb_bits = nb_bytes * 8;

        if nb_bits % 32 != 0 {
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

    pub fn generate_mnemonic<R>(rng: &mut R) -> Result<Self, Bip39Error>
    where
        R: rand::RngCore + rand::CryptoRng,
    {
        let mut entropy = [0u8; STRENGTH];
        rand::RngCore::fill_bytes(rng, &mut entropy);

        Self::entropy_to_mnemonic(&entropy)
    }

    pub fn validate_mnemonic(&self) -> bool {
        let words = self.get();

        match Self::mnemonic_to_entropy(&words) {
            Ok(_) => return true,
            Err(_) => return false,
        }
    }

    pub fn get_seed(&self, password: &str) -> [u8; 64] {
        let binding = self.get();
        let mnemonic_bytes = binding.as_bytes();
        let salt = self.to_salt(password);

        pbkdf2_hmac_array::<Sha512, 64>(&mnemonic_bytes, salt.as_bytes(), NUMBER_WORDS as u32)
    }

    pub fn get_list(&self) -> Vec<&str> {
        match self {
            Mnemonic::English(points) => points
                .iter()
                .map(|i| language::english::WORDS[*i as usize])
                .collect::<Vec<&str>>(),
        }
    }

    pub fn get(&self) -> String {
        let mut words = Cow::from(self.get_list().join(" "));
        normalize_utf8_cow(&mut words);

        words.to_string()
    }

    fn to_salt(&self, password: &str) -> String {
        let mut password = password.to_string();

        password.push_str(SALT_PREFIX);

        let mut password = Cow::from(password);

        normalize_utf8_cow(&mut password);

        password.to_string()
    }
}

#[test]
fn test_mnemonic() {
    use rand;

    let mut rng = rand::thread_rng();
    let m = Mnemonic::generate_mnemonic(&mut rng).unwrap();

    assert_eq!(m.get_list().len(), SIZE);

    let m0 = Mnemonic::generate_mnemonic(&mut rng).unwrap();

    let is_valid = m.validate_mnemonic();
    let is_valid0 = m0.validate_mnemonic();

    assert!(is_valid);
    assert!(is_valid0);
}