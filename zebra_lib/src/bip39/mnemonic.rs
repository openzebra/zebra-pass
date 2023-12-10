//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use crate::errors::ZebraErrors;

use super::{
    config::{MAX_NB_WORDS, MIN_NB_WORDS, NUMBER_WORDS, SALT_PREFIX},
    language,
};
use pbkdf2::pbkdf2_hmac_array;
use sha2::{Digest, Sha256, Sha512};
use std::borrow::Cow;
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone)]
pub enum Language {
    English,
}

#[derive(Debug)]
pub struct Mnemonic {
    pub indicators: [u16; MAX_NB_WORDS],
    pub lang: Language,
    pub size: usize,
}

fn normalize_utf8_cow<'a>(cow: &mut Cow<'a, str>) {
    let is_nfkd = unicode_normalization::is_nfkd_quick(cow.as_ref().chars());
    if is_nfkd != unicode_normalization::IsNormalized::Yes {
        *cow = Cow::Owned(cow.as_ref().nfkd().to_string());
    }
}

fn is_invalid_word_count(word_count: usize) -> bool {
    word_count < MIN_NB_WORDS || word_count % 3 != 0 || word_count > MAX_NB_WORDS
}

impl Mnemonic {
    pub fn entropy_to_mnemonic(
        language: Language,
        entropy: &[u8],
        size: usize,
    ) -> Result<Mnemonic, ZebraErrors> {
        const MAX_ENTROPY_BITS: usize = 256;
        const MIN_ENTROPY_BITS: usize = 128;
        const MAX_CHECKSUM_BITS: usize = 8;

        let nb_bytes = entropy.len();
        let nb_bits = nb_bytes * 8;

        if nb_bits % 32 != 0 {
            return Err(ZebraErrors::Bip39BadEntropyBitCount(nb_bits));
        }
        if nb_bits < MIN_ENTROPY_BITS || nb_bits > MAX_ENTROPY_BITS {
            return Err(ZebraErrors::Bip39BadEntropyBitCount(nb_bits));
        }

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

        let mut words = [u16::MAX; MAX_NB_WORDS];
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

        Ok(Mnemonic {
            indicators: words,
            lang: language,
            size,
        })
    }

    pub fn mnemonic_to_entropy(
        language: Language,
        mnemonic: &str,
    ) -> Result<Mnemonic, ZebraErrors> {
        let nb_words = mnemonic.split_whitespace().count();

        if is_invalid_word_count(nb_words) {
            return Err(ZebraErrors::Bip39BadWordCount(nb_words));
        }

        let mut words = [u16::MAX; MAX_NB_WORDS];
        let mut bits = [false; MAX_NB_WORDS * 11];

        for (i, word) in mnemonic.split_whitespace().enumerate() {
            let idx = language::english::WORDS
                .iter()
                .position(|w| *w == word)
                .ok_or(ZebraErrors::Bip39UnknownWord(i))?;

            words[i] = idx as u16;

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

        let mut hasher = Sha256::new();
        hasher.update(&entropy[0..nb_bytes_entropy]);
        let check = hasher.finalize();

        for i in 0..nb_bytes_entropy / 4 {
            if bits[8 * nb_bytes_entropy + i] != ((check[i / 8] & (1 << (7 - (i % 8)))) > 0) {
                return Err(ZebraErrors::Bip39InvalidChecksum);
            }
        }

        Ok(Mnemonic {
            lang: language,
            indicators: words,
            size: nb_words,
        })
    }

    pub fn gen<R>(rng: &mut R, size: usize, lang: Language) -> Result<Self, ZebraErrors>
    where
        R: rand::RngCore + rand::CryptoRng,
    {
        let mut entropy = vec![0u8; (size / 3) * 4];

        rand::RngCore::fill_bytes(rng, &mut entropy);

        Self::entropy_to_mnemonic(lang, &entropy, size)
    }

    pub fn validate(words: &str) -> bool {
        // TODO: make lang detect
        match Self::mnemonic_to_entropy(Language::English, &words) {
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

    pub fn get(&self) -> String {
        let mut words = Cow::from(self.get_vec().join(" "));
        normalize_utf8_cow(&mut words);

        words.to_string()
    }

    pub fn get_vec(&self) -> Vec<&str> {
        let mut out = Vec::with_capacity(self.size);

        match self.lang {
            Language::English => {
                for i in 0..self.size {
                    out.push(language::english::WORDS[self.indicators[i] as usize]);
                }
            }
        };

        out
    }

    fn to_salt(&self, password: &str) -> String {
        let mut password = password.to_string();

        password.push_str(SALT_PREFIX);

        let mut password = Cow::from(password);

        normalize_utf8_cow(&mut password);

        password.to_string()
    }
}

#[cfg(test)]
mod test_bip39_mnemonic {
    use super::*;
    use rand;

    #[test]
    fn test_invalid_words() {
        let words =
            "getter advice cage absurd amount doctor acoustic avoid letter advice cage above";
        let r = Mnemonic::mnemonic_to_entropy(Language::English, &words);

        assert!(r.is_err());
    }

    #[test]
    fn test_gen_12_words() {
        const SIZE: usize = 12;
        let mut rng = rand::thread_rng();
        let m = Mnemonic::gen(&mut rng, SIZE, Language::English).unwrap();
        let words = m.get();

        assert!(Mnemonic::validate(&words));
        assert!(m.get_vec().len() == SIZE);
    }

    #[test]
    fn test_gen_15_words() {
        const SIZE: usize = 15;
        let mut rng = rand::thread_rng();
        let m = Mnemonic::gen(&mut rng, SIZE, Language::English).unwrap();
        let words = m.get();

        assert!(Mnemonic::validate(&words));
        assert!(m.get_vec().len() == SIZE);
    }

    #[test]
    fn test_gen_18_words() {
        const SIZE: usize = 18;
        let mut rng = rand::thread_rng();
        let m = Mnemonic::gen(&mut rng, SIZE, Language::English).unwrap();
        let words = m.get();

        assert!(Mnemonic::validate(&words));
        assert!(m.get_vec().len() == SIZE);
    }

    #[test]
    fn test_gen_21_words() {
        const SIZE: usize = 21;
        let mut rng = rand::thread_rng();
        let m = Mnemonic::gen(&mut rng, SIZE, Language::English).unwrap();
        let words = m.get();

        assert!(Mnemonic::validate(&words));
        assert!(m.get_vec().len() == SIZE);
    }

    #[test]
    fn test_gen_24_words() {
        const SIZE: usize = 24;
        let mut rng = rand::thread_rng();
        let m = Mnemonic::gen(&mut rng, SIZE, Language::English).unwrap();
        let words = m.get();

        assert!(Mnemonic::validate(&words));
        assert!(m.get_vec().len() == SIZE);
    }
}
