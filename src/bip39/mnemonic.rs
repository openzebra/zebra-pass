//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::borrow::Cow;

use pbkdf2::pbkdf2_hmac_array;
use sha2::Sha512;
use unicode_normalization::UnicodeNormalization;

use super::language::Language;

const SALT_PREFIX: &str = "zebra-bip39-mnemonic";

pub struct Mnemonic {
    lang: Language,
    words: [u16; 24],
}

impl Mnemonic {
    pub fn normalize_utf8_cow<'a>(cow: &mut Cow<'a, str>) {
        let is_nfkd = unicode_normalization::is_nfkd_quick(cow.as_ref().chars());
        if is_nfkd != unicode_normalization::IsNormalized::Yes {
            *cow = Cow::Owned(cow.as_ref().nfkd().to_string());
        }
    }
}

#[test]
fn test_pbkdf2() {
    let password = b"password";
    let salt = b"salt";
    let n = 2048;
    let key2 = pbkdf2_hmac_array::<Sha512, 20>(password, salt, n);

    println!("{:?}", key2);
}
