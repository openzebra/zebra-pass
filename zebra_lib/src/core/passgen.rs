//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use rand::seq::SliceRandom;

use crate::errors::ZebraErrors;

const CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMSET: &str = "0123456789";
const SYMBOLSSET: &str = "!@#$%^&*()_+-=";

#[derive(Debug)]
pub struct PassGen {
    pub lowercase: bool,
    pub upercase: bool,
    pub nums: bool,
    pub symbols: bool,
}

impl PassGen {
    pub fn from(lowercase: bool, upercase: bool, nums: bool, symbols: bool) -> Self {
        Self {
            lowercase,
            upercase,
            nums,
            symbols,
        }
    }

    pub fn gen<R>(&self, length: usize, rng: &mut R) -> Result<Vec<u8>, ZebraErrors>
    where
        R: rand::RngCore + rand::CryptoRng,
    {
        let mut all_sets = vec![];
        let mut out = Vec::with_capacity(length);

        if self.lowercase {
            all_sets.push(CHARSET.to_lowercase());
        }

        if self.upercase {
            all_sets.push(CHARSET.to_string());
        }

        if self.nums {
            all_sets.push(NUMSET.to_string());
        }

        if self.symbols {
            all_sets.push(SYMBOLSSET.to_string());
        }

        for _ in 0..length {
            let random_set = all_sets.choose(rng).ok_or(ZebraErrors::PassGenInvalidRng)?;
            let random_char = random_set
                .as_bytes()
                .choose(rng)
                .ok_or(ZebraErrors::PassGenInvalidRng)?;

            out.push(*random_char);
        }

        Ok(out)
    }
}

impl Default for PassGen {
    fn default() -> Self {
        PassGen::from(true, true, true, true)
    }
}

#[test]
fn test_pass_gen() {
    let mut rng = rand::thread_rng();
    let gen = PassGen::from(true, true, true, true);

    let random_pass = gen.gen(30, &mut rng).unwrap();

    for x in random_pass {
        assert!(
            CHARSET.as_bytes().contains(&x)
                || CHARSET.to_lowercase().as_bytes().contains(&x)
                || NUMSET.as_bytes().contains(&x)
                || SYMBOLSSET.as_bytes().contains(&x)
        );
    }
}
