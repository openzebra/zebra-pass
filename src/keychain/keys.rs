//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::sync::Arc;

use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes256;
use ntrulp::key::{priv_key::PrivKey, pub_key::PubKey};
use ntrulp::ntru;
use ntrulp::ntru::errors::NTRUErrors;
use ntrulp::params::params1277::{PUBLICKEYS_BYTES, SECRETKEYS_BYTES};
use ntrulp::poly::r3::R3;
use ntrulp::poly::rq::Rq;
use ntrulp::random::{CommonRandom, NTRURandom};
use num_cpus;
use pbkdf2::pbkdf2_hmac_array;
use serde::{Deserialize, Serialize};
use sha2::Sha512;

use crate::bip39::mnemonic::Mnemonic;

use super::errors::KeyChainErrors;

const PASSWORD_SALT: [u8; 16] = [
    131, 53, 247, 96, 233, 128, 223, 191, 171, 58, 191, 97, 236, 210, 100, 70,
];
// TODO: possible to add settings.
const DIFFICULTY: u32 = 2048;
const SHA512_SIZE: usize = 64;
const SHA256_SIZE: usize = SHA512_SIZE / 2;
const AES_BLOCK_SIZE: usize = 16;

#[derive(Debug, Deserialize, Serialize)]
pub enum CipherOrders {
    AES256,
    NTRUP1277,
}

pub struct KeyChain {
    pub ntrup_keys: (Arc<PrivKey>, Arc<PubKey>),
    // TODO: Remake it to TwoFish
    pub aes_key: [u8; SHA256_SIZE],
    num_threads: usize,
}

fn gen_from_seed(
    seed_bytes: [u8; SHA512_SIZE],
) -> Result<([u8; SHA256_SIZE], PubKey, PrivKey), KeyChainErrors> {
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

    Ok((aes_key, pk, sk))
}

impl KeyChain {
    pub fn from_pass(password: &[u8]) -> Result<Self, KeyChainErrors> {
        let seed_bytes =
            pbkdf2_hmac_array::<Sha512, SHA512_SIZE>(password, &PASSWORD_SALT, DIFFICULTY);
        let (aes_key, pk, sk) = gen_from_seed(seed_bytes)?;
        let num_threads = num_cpus::get();

        Ok(Self {
            ntrup_keys: (Arc::new(sk), Arc::new(pk)),
            aes_key,
            num_threads,
        })
    }

    pub fn from_bip39(words: &str, password: &str) -> Result<Self, KeyChainErrors> {
        if !Mnemonic::validate_mnemonic(words) {
            return Err(KeyChainErrors::InvalidMnemonic);
        }

        let m = Mnemonic::mnemonic_to_entropy(&words).or(Err(KeyChainErrors::InvalidMnemonic))?;
        let num_threads = num_cpus::get();
        let seed_bytes = m.get_seed(password);
        let (aes_key, pk, sk) = gen_from_seed(seed_bytes)?;

        Ok(Self {
            ntrup_keys: (Arc::new(sk), Arc::new(pk)),
            aes_key,
            num_threads,
        })
    }

    pub fn from_keys(
        key: [u8; SHA256_SIZE],
        pqsk: [u8; SECRETKEYS_BYTES],
        pqpk: [u8; PUBLICKEYS_BYTES],
    ) -> Result<Self, KeyChainErrors> {
        let num_threads = num_cpus::get();
        let secret_key = PrivKey::import(&pqsk).or(Err(KeyChainErrors::SKError))?;
        let pub_key = PubKey::import(&pqpk).or(Err(KeyChainErrors::PKError))?;

        Ok(Self {
            num_threads,
            ntrup_keys: (Arc::new(secret_key), Arc::new(pub_key)),
            aes_key: key,
        })
    }

    pub fn encrypt(&self, bytes: Vec<u8>) -> Result<(Vec<u8>, [CipherOrders; 2]), KeyChainErrors> {
        let options = [CipherOrders::NTRUP1277, CipherOrders::AES256];
        let mut tmp = bytes;

        for o in &options {
            match o {
                CipherOrders::AES256 => tmp = self.aes_encrypt(&tmp),
                CipherOrders::NTRUP1277 => {
                    tmp = self
                        .ntru_encrypt(&Arc::new(tmp))
                        .or(Err(KeyChainErrors::NTRUEncryptError))?
                }
            };
        }

        Ok((tmp, options))
    }

    pub fn decrypt(
        &self,
        bytes: Vec<u8>,
        options: [CipherOrders; 2],
    ) -> Result<Vec<u8>, KeyChainErrors> {
        let mut tmp = bytes;

        for o in options.iter().rev() {
            match o {
                CipherOrders::AES256 => tmp = self.aes_decrypt(&tmp)?,
                CipherOrders::NTRUP1277 => {
                    tmp = self
                        .ntru_decrypt(&Arc::new(tmp))
                        .or(Err(KeyChainErrors::NTRUDecryptError))?;
                }
            };
        }

        Ok(tmp)
    }

    fn aes_decrypt(&self, bytes: &[u8]) -> Result<Vec<u8>, KeyChainErrors> {
        let key = GenericArray::from(self.aes_key);
        let cipher = Aes256::new(&key);
        let point_bytes: [u8; 8] = bytes[bytes.len() - 8..]
            .try_into()
            .or(Err(KeyChainErrors::SliceError))?;
        let point = usize::from_be_bytes(point_bytes);
        let mut blocks = Vec::new();

        for chunk in bytes[..bytes.len() - 8].chunks(AES_BLOCK_SIZE) {
            let block: [u8; AES_BLOCK_SIZE] =
                chunk.try_into().or(Err(KeyChainErrors::SliceError))?;
            blocks.push(GenericArray::from(block));
        }

        cipher.decrypt_blocks(&mut blocks);

        let mut decrypted = Vec::new();

        for block in blocks {
            decrypted.extend(block);
        }

        if point != 0 {
            decrypted = decrypted[..decrypted.len() - AES_BLOCK_SIZE + point].to_vec();
        }

        Ok(decrypted)
    }

    fn ntru_decrypt(&self, bytes: &Arc<Vec<u8>>) -> Result<Vec<u8>, NTRUErrors> {
        let sk = &self.ntrup_keys.0;

        ntru::cipher::parallel_bytes_decrypt(&bytes, &sk, self.num_threads)
    }

    fn aes_encrypt(&self, bytes: &[u8]) -> Vec<u8> {
        let key = GenericArray::from(self.aes_key);
        let cipher = Aes256::new(&key);
        let mut blocks = Vec::new();
        let mut pointer = (0_usize).to_be_bytes();

        for chunk in bytes.chunks(AES_BLOCK_SIZE) {
            if chunk.len() == AES_BLOCK_SIZE {
                let block: [u8; AES_BLOCK_SIZE] = chunk.try_into().unwrap();

                blocks.push(GenericArray::from(block));
            } else {
                let mut block = [0u8; AES_BLOCK_SIZE];

                for i in 0..AES_BLOCK_SIZE {
                    match chunk.get(i) {
                        Some(v) => block[i] = *v,
                        None => {
                            pointer = i.to_be_bytes();
                            break;
                        }
                    }
                }

                blocks.push(GenericArray::from(block));
            }
        }

        cipher.encrypt_blocks(&mut blocks);

        let mut encrypted = Vec::new();

        for chunk in blocks {
            encrypted.extend(chunk);
        }

        encrypted.extend(pointer);

        encrypted
    }

    fn ntru_encrypt(&self, bytes: &Arc<Vec<u8>>) -> Result<Vec<u8>, NTRUErrors> {
        let mut rng = NTRURandom::new();
        let bytes = Arc::new(bytes);
        let pk = &self.ntrup_keys.1;

        ntru::cipher::parallel_bytes_encrypt(&mut rng, &bytes, &pk, self.num_threads)
    }
}

#[cfg(test)]
mod test_key_chain {
    use super::*;
    use rand;
    use rand::RngCore;

    #[test]
    fn test_aes_encrypt_decrypt() {
        let mut rng = rand::thread_rng();
        let mut password = [0u8; 2000];
        let mut ciphertext = vec![0u8; 1233];

        rng.fill_bytes(&mut password);
        rng.fill_bytes(&mut ciphertext);

        let keys = KeyChain::from_pass(&password).unwrap();

        let encrypted = keys.aes_encrypt(&ciphertext);
        let decrypted = keys.aes_decrypt(&encrypted).unwrap();

        assert_eq!(decrypted, ciphertext);
    }

    #[test]
    fn test_pq_encrypt_decrypt() {
        let mut rng = rand::thread_rng();
        let mut password = [0u8; 2000];
        let ciphertext = Arc::new(vec![42u8; 1233]);

        rng.fill_bytes(&mut password);

        let keys = KeyChain::from_pass(&password).unwrap();

        let encrypted = keys.ntru_encrypt(&ciphertext).unwrap();
        let decrypted = keys.ntru_decrypt(&Arc::new(encrypted)).unwrap();

        assert_eq!(decrypted, ciphertext.to_vec());
    }

    #[test]
    fn test_encrypt_decrypt() {
        let mut rng = rand::thread_rng();
        let mut password = [0u8; 2000];
        let mut ciphertext = vec![42u8; 1233];

        rng.fill_bytes(&mut password);
        rng.fill_bytes(&mut ciphertext);

        let keys = KeyChain::from_pass(&password).unwrap();

        let (encrypted, options) = keys.encrypt(ciphertext.clone()).unwrap();
        let decrypted = keys.decrypt(encrypted, options).unwrap();

        assert_eq!(decrypted, ciphertext);
    }

    #[test]
    fn test_key_chain_init() {
        let mut rng = rand::thread_rng();
        let mut password = [0u8; 2000];

        rng.fill_bytes(&mut password);

        let keys0 = KeyChain::from_pass(&password);

        assert!(keys0.is_ok());

        let keys1 = KeyChain::from_pass(&password);

        assert!(keys1.is_ok());

        let keys1 = keys1.unwrap();
        let keys0 = keys0.unwrap();

        assert_eq!(keys1.aes_key, keys0.aes_key);
        assert_eq!(keys1.ntrup_keys.0 .0.coeffs, keys0.ntrup_keys.0 .0.coeffs);
        assert_eq!(keys1.ntrup_keys.0 .1.coeffs, keys0.ntrup_keys.0 .1.coeffs);
        assert_eq!(keys1.ntrup_keys.1.coeffs, keys0.ntrup_keys.1.coeffs);
    }

    #[test]
    fn te_keychain_bip39() {
        let mut rng = rand::thread_rng();
        let m = Mnemonic::generate_mnemonic(&mut rng).unwrap();
        let words = m.get();
        let password = "test-password";
        let keys0 = KeyChain::from_bip39(&words, password).unwrap();
        let keys1 = KeyChain::from_bip39(&words, password).unwrap();

        assert_eq!(keys1.aes_key, keys0.aes_key);
        assert_eq!(keys1.ntrup_keys.0 .0.coeffs, keys0.ntrup_keys.0 .0.coeffs);
        assert_eq!(keys1.ntrup_keys.0 .1.coeffs, keys0.ntrup_keys.0 .1.coeffs);
        assert_eq!(keys1.ntrup_keys.1.coeffs, keys0.ntrup_keys.1.coeffs);
    }
}
