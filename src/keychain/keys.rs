//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
extern crate hex;

use std::sync::Arc;

use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes256;
use ntrulp::key::{priv_key::PrivKey, pub_key::PubKey};
use ntrulp::ntru;
use ntrulp::params::params1277::{PUBLICKEYS_BYTES, SECRETKEYS_BYTES};
use ntrulp::poly::r3::R3;
use ntrulp::poly::rq::Rq;
use ntrulp::random::{CommonRandom, NTRURandom};
use num_cpus;
use pbkdf2::pbkdf2_hmac_array;
use serde::{Deserialize, Serialize};
use sha2::Sha512;

use crate::bip39::mnemonic::Mnemonic;
use crate::errors::ZebraErrors;

const PASSWORD_SALT: [u8; 16] = [
    131, 53, 247, 96, 233, 128, 223, 191, 171, 58, 191, 97, 236, 210, 100, 70,
];
const SHA512_SIZE: usize = 64;
const SHA256_SIZE: usize = SHA512_SIZE / 2;
const AES_BLOCK_SIZE: usize = 16;
pub const AES_KEY_SIZE: usize = 32;

#[derive(Debug, Clone, Deserialize, Serialize)]
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
) -> Result<([u8; SHA256_SIZE], PubKey, PrivKey), ZebraErrors> {
    let seed_pq: [u8; 8] = seed_bytes[..8]
        .try_into()
        .or(Err(ZebraErrors::KeyChainSliceError))?;
    let aes_key: [u8; SHA256_SIZE] = seed_bytes[SHA256_SIZE..]
        .try_into()
        .or(Err(ZebraErrors::KeyChainSliceError))?;

    // TODO: make it as seed from 32 byts.
    let pq_seed_u64 = u64::from_be_bytes(seed_pq);
    let mut pq_rng = NTRURandom::from_u64(pq_seed_u64);
    let f: Rq = Rq::from(
        pq_rng
            .short_random()
            .or(Err(ZebraErrors::KeyChainNTRURngError))?,
    );
    let mut g: R3;
    let sk = loop {
        // TODO: this can be endless.
        let r = pq_rng
            .random_small()
            .or(Err(ZebraErrors::KeyChainNTRURngError))?;
        g = R3::from(r);

        match PrivKey::compute(&f, &g) {
            Ok(s) => break s,
            Err(_) => continue,
        };
    };
    let pk = PubKey::compute(&f, &g).or(Err(ZebraErrors::KeyChainGenNTRUKeysError))?;

    Ok((aes_key, pk, sk))
}

impl KeyChain {
    pub fn from_pass(password: &[u8], difficulty: u32) -> Result<Self, ZebraErrors> {
        let seed_bytes =
            pbkdf2_hmac_array::<Sha512, SHA512_SIZE>(password, &PASSWORD_SALT, difficulty);
        let (aes_key, pk, sk) = gen_from_seed(seed_bytes)?;
        let num_threads = num_cpus::get();

        Ok(Self {
            ntrup_keys: (Arc::new(sk), Arc::new(pk)),
            aes_key,
            num_threads,
        })
    }

    pub fn from_bip39(words: &str, password: &str) -> Result<Self, ZebraErrors> {
        if !Mnemonic::validate_mnemonic(words) {
            return Err(ZebraErrors::Bip39InvalidMnemonic);
        }

        let m = Mnemonic::mnemonic_to_entropy(&words)?;
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
    ) -> Result<Self, ZebraErrors> {
        let num_threads = num_cpus::get();
        let secret_key = PrivKey::import(&pqsk).or(Err(ZebraErrors::KeyChainNTRUImportSKError))?;
        let pub_key = PubKey::import(&pqpk).or(Err(ZebraErrors::KeyChainNTRUImportPKError))?;

        Ok(Self {
            num_threads,
            ntrup_keys: (Arc::new(secret_key), Arc::new(pub_key)),
            aes_key: key,
        })
    }

    pub fn as_bytes(&self) -> [u8; AES_KEY_SIZE + SECRETKEYS_BYTES + PUBLICKEYS_BYTES] {
        let mut out = [0u8; AES_KEY_SIZE + SECRETKEYS_BYTES + PUBLICKEYS_BYTES];
        let (sk, pk) = &self.ntrup_keys;

        out[..AES_KEY_SIZE].copy_from_slice(&self.aes_key);
        out[AES_KEY_SIZE..PUBLICKEYS_BYTES + AES_KEY_SIZE].copy_from_slice(&pk.as_bytes());
        out[AES_KEY_SIZE + PUBLICKEYS_BYTES..].copy_from_slice(&sk.as_bytes());

        out
    }

    pub fn encrypt(&self, bytes: Vec<u8>, options: &[CipherOrders]) -> Result<String, ZebraErrors> {
        let mut tmp = bytes;

        for o in options {
            match o {
                CipherOrders::AES256 => tmp = self.aes_encrypt(&tmp),
                CipherOrders::NTRUP1277 => tmp = self.ntru_encrypt(&Arc::new(tmp))?,
            };
        }

        let content = hex::encode(tmp);

        Ok(content)
    }

    pub fn decrypt(&self, data: &str, options: &[CipherOrders]) -> Result<Vec<u8>, ZebraErrors> {
        let mut tmp = hex::decode(data).or(Err(ZebraErrors::KeychainDataIsNotHex))?;

        for o in options.iter().rev() {
            match o {
                CipherOrders::AES256 => tmp = self.aes_decrypt(&tmp)?,
                CipherOrders::NTRUP1277 => tmp = self.ntru_decrypt(&Arc::new(tmp))?,
            };
        }

        Ok(tmp)
    }

    fn aes_decrypt(&self, bytes: &[u8]) -> Result<Vec<u8>, ZebraErrors> {
        let key = GenericArray::from(self.aes_key);
        let cipher = Aes256::new(&key);
        let point_bytes: [u8; 8] = bytes[bytes.len() - 8..]
            .try_into()
            .or(Err(ZebraErrors::KeyChainSliceError))?;
        let point = usize::from_be_bytes(point_bytes);
        let mut blocks = Vec::new();

        for chunk in bytes[..bytes.len() - 8].chunks(AES_BLOCK_SIZE) {
            let block: [u8; AES_BLOCK_SIZE] =
                chunk.try_into().or(Err(ZebraErrors::KeyChainSliceError))?;
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

    fn ntru_decrypt(&self, bytes: &Arc<Vec<u8>>) -> Result<Vec<u8>, ZebraErrors> {
        let sk = &self.ntrup_keys.0;

        ntru::cipher::parallel_bytes_decrypt(&bytes, &sk, self.num_threads)
            .or(Err(ZebraErrors::KeychainDataDecryptError))
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

    fn ntru_encrypt(&self, bytes: &Arc<Vec<u8>>) -> Result<Vec<u8>, ZebraErrors> {
        let mut rng = NTRURandom::new();
        let bytes = Arc::new(bytes);
        let pk = &self.ntrup_keys.1;

        ntru::cipher::parallel_bytes_encrypt(&mut rng, &bytes, &pk, self.num_threads)
            .or(Err(ZebraErrors::KeychainDataEncryptError))
    }
}

#[cfg(test)]
mod test_key_chain {
    use super::*;
    use rand;
    use rand::RngCore;

    const DIFFICULTY: u32 = 1024;

    #[test]
    fn test_aes_encrypt_decrypt() {
        let mut rng = rand::thread_rng();
        let mut password = [0u8; 2000];
        let mut ciphertext = vec![0u8; 1233];

        rng.fill_bytes(&mut password);
        rng.fill_bytes(&mut ciphertext);

        let keys = KeyChain::from_pass(&password, DIFFICULTY).unwrap();

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

        let keys = KeyChain::from_pass(&password, DIFFICULTY).unwrap();

        let encrypted = keys.ntru_encrypt(&ciphertext).unwrap();
        let decrypted = keys.ntru_decrypt(&Arc::new(encrypted)).unwrap();

        assert_eq!(decrypted, ciphertext.to_vec());
    }

    #[test]
    fn test_export_keys() {
        let mut rng = rand::thread_rng();
        let mut password = [0u8; 2000];
        let mut ciphertext = vec![42u8; 1233];

        rng.fill_bytes(&mut password);
        rng.fill_bytes(&mut ciphertext);

        let keys = KeyChain::from_pass(&password, DIFFICULTY).unwrap();
        let keys_bytes = keys.as_bytes();

        assert_eq!(keys_bytes[..AES_KEY_SIZE], keys.aes_key);
        assert_eq!(
            keys_bytes[AES_KEY_SIZE + PUBLICKEYS_BYTES..],
            keys.ntrup_keys.0.as_bytes()
        );
        assert_eq!(
            keys_bytes[AES_KEY_SIZE..AES_KEY_SIZE + PUBLICKEYS_BYTES],
            keys.ntrup_keys.1.as_bytes()
        );
    }

    #[test]
    fn test_encrypt_decrypt() {
        let mut rng = rand::thread_rng();
        let mut password = [0u8; 2000];
        let mut ciphertext = vec![42u8; 1233];

        rng.fill_bytes(&mut password);
        rng.fill_bytes(&mut ciphertext);

        let keys = KeyChain::from_pass(&password, DIFFICULTY).unwrap();

        let orders = vec![CipherOrders::NTRUP1277, CipherOrders::AES256];
        let secure_data = keys.encrypt(ciphertext.clone(), &orders).unwrap();
        let decrypted = keys.decrypt(&secure_data, &orders).unwrap();

        assert_eq!(decrypted, ciphertext);
    }

    #[test]
    fn test_key_chain_init() {
        let mut rng = rand::thread_rng();
        let mut password = [0u8; 2000];

        rng.fill_bytes(&mut password);

        let keys0 = KeyChain::from_pass(&password, DIFFICULTY);

        assert!(keys0.is_ok());

        let keys1 = KeyChain::from_pass(&password, DIFFICULTY);

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
