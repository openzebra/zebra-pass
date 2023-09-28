//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::borrow::Cow;

use pbkdf2::pbkdf2_hmac_array;
use sha2::{Digest, Sha256, Sha512};
use unicode_normalization::UnicodeNormalization;

use super::{errors::Bip39Error, language::Language};

const SALT_PREFIX: &str = "zebra-bip39-mnemonic";
const EOF: u16 = u16::max_value();
const MAX_NB_WORDS: usize = 24;
const MIN_NB_WORDS: usize = 12;

pub struct Mnemonic {
    pub lang: Language,
    pub words: [u16; 24],
}

fn is_invalid_word_count(word_count: usize) -> bool {
    word_count < MIN_NB_WORDS || word_count % 3 != 0 || word_count > MAX_NB_WORDS
}

fn normalize_utf8_cow<'a>(cow: &mut Cow<'a, str>) {
    let is_nfkd = unicode_normalization::is_nfkd_quick(cow.as_ref().chars());
    if is_nfkd != unicode_normalization::IsNormalized::Yes {
        *cow = Cow::Owned(cow.as_ref().nfkd().to_string());
    }
}

impl Mnemonic {
    pub fn from_entropy_in(lang: Language, entropy: &[u8]) -> Result<Mnemonic, Bip39Error> {
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

        let mut words = [EOF; MAX_NB_WORDS];
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

        Ok(Mnemonic { words, lang })
    }

    pub fn from_entropy(entropy: &[u8]) -> Result<Mnemonic, Bip39Error> {
        Mnemonic::from_entropy_in(Language::English, entropy)
    }

    pub fn generate_in_with<R>(
        rng: &mut R,
        language: Language,
        word_count: usize,
    ) -> Result<Mnemonic, Bip39Error>
    where
        R: rand::RngCore + rand::CryptoRng,
    {
        if is_invalid_word_count(word_count) {
            return Err(Bip39Error::BadWordCount(word_count));
        }

        let entropy_bytes = (word_count / 3) * 4;
        let mut entropy = [0u8; (MAX_NB_WORDS / 3) * 4];

        rand::RngCore::fill_bytes(rng, &mut entropy[0..entropy_bytes]);
        Mnemonic::from_entropy_in(language, &entropy[0..entropy_bytes])
    }

    pub fn generate(lang: Language, word_count: usize) -> Result<Mnemonic, Bip39Error> {
        Mnemonic::generate_in_with(&mut rand::thread_rng(), lang, word_count)
    }

    pub fn parse_in_normalized(lang: Language, s: &str) -> Result<Mnemonic, Bip39Error> {
        let nb_words = s.split_whitespace().count();
        if is_invalid_word_count(nb_words) {
            return Err(Bip39Error::BadWordCount(nb_words));
        }

        let mut words = [EOF; MAX_NB_WORDS];
        let mut bits = [false; MAX_NB_WORDS * 11];

        for (i, word) in s.split_whitespace().enumerate() {
            let idx = lang.find_word(word).ok_or(Bip39Error::UnknownWord(i))?;

            words[i] = idx;

            for j in 0..11 {
                bits[i * 11 + j] = idx >> (10 - j) & 1 == 1;
            }
        }

        let mut entropy = [0u8; MAX_NB_WORDS / 3 * 4];
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

        Ok(Mnemonic { lang, words })
    }

    pub fn to_seed(&self, password: &str) -> [u8; 64] {
        self.to_seed_normalized(&self.salt(password))
    }

    fn to_seed_normalized(&self, salt: &[u8]) -> [u8; 64] {
        let mut mnemonic = Cow::from("");

        normalize_utf8_cow(&mut mnemonic);

        let seed: [u8; 64] = pbkdf2_hmac_array::<Sha512, 64>(mnemonic.as_bytes(), salt, 2048);

        seed
    }

    fn salt(&self, password: &str) -> Vec<u8> {
        let mut cow = Cow::from(password.to_string() + &SALT_PREFIX.to_string());

        normalize_utf8_cow(&mut cow);

        cow.as_bytes().to_vec()
    }
}
