//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use ntrulp::key::{priv_key::PrivKey, pub_key::PubKey};
use ntrulp::poly::r3::R3;
use ntrulp::poly::rq::Rq;
use ntrulp::random::{CommonRandom, NTRURandom};
use pbkdf2::pbkdf2_hmac_array;
use sha2::Sha512;

use super::errors::KeyChainErrors;

const PASSWORD_SALT: [u8; 16] = [
    131, 53, 247, 96, 233, 128, 223, 191, 171, 58, 191, 97, 236, 210, 100, 70,
];
const DIFFICULTY: u32 = 16000;
const SHA512_SIZE: usize = 64;
const SHA256_SIZE: usize = SHA512_SIZE / 2;

pub struct KeyChain {
    pub ntrup_keys: (PrivKey, PubKey),
    // TODO: Remake it to TwoFish
    pub aes_key: [u8; SHA256_SIZE],
}

impl KeyChain {
    pub fn from(password: &[u8]) -> Result<Self, KeyChainErrors> {
        let seed_bytes =
            pbkdf2_hmac_array::<Sha512, SHA512_SIZE>(password, &PASSWORD_SALT, DIFFICULTY);
        let seed_pq: [u8; 8] = seed_bytes[..8]
            .try_into()
            .or(Err(KeyChainErrors::SliceError))?;
        let aes_key: [u8; SHA256_SIZE] = seed_bytes[SHA256_SIZE..]
            .try_into()
            .or(Err(KeyChainErrors::SliceError))?;

        // TODO: make it as seed from 32 byts.
        let pq_seed_u64 = u64::from_be_bytes(seed_pq);
        let mut pq_rng = NTRURandom::from_u64(pq_seed_u64);
        let f: Rq = Rq::from(pq_rng.short_random().or(Err(KeyChainErrors::RngError))?);
        let mut g: R3;
        let sk = loop {
            // TODO: this can be endless.
            let r = pq_rng.random_small().or(Err(KeyChainErrors::RngError))?;
            g = R3::from(r);

            match PrivKey::compute(&f, &g) {
                Ok(s) => break s,
                Err(_) => continue,
            };
        };
        let pk = PubKey::compute(&f, &g).or(Err(KeyChainErrors::GenKeysError))?;

        Ok(Self {
            ntrup_keys: (sk, pk),
            aes_key,
        })
    }
}

#[test]
fn test_key_chain() {
    use rand;
    use rand::RngCore;

    let mut rng = rand::thread_rng();
    let mut password = [0u8; 2000];

    rng.fill_bytes(&mut password);

    let keys0 = KeyChain::from(&password);

    assert!(keys0.is_ok());

    let keys1 = KeyChain::from(&password);

    assert!(keys1.is_ok());

    let keys1 = keys1.unwrap();
    let keys0 = keys0.unwrap();

    assert_eq!(keys1.aes_key, keys0.aes_key);
    assert_eq!(keys1.ntrup_keys.0 .0.coeffs, keys0.ntrup_keys.0 .0.coeffs);
    assert_eq!(keys1.ntrup_keys.0 .1.coeffs, keys0.ntrup_keys.0 .1.coeffs);
    assert_eq!(keys1.ntrup_keys.1.coeffs, keys0.ntrup_keys.1.coeffs);
}
