//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use crate::{
    bip39::mnemonic::{Language, Mnemonic},
    errors::ZebraErrors,
};
use slint::{Model, ModelRc, SharedString, VecModel};

pub fn gen_bip39_words(m: &Mnemonic, chunk_size: usize) -> ModelRc<ModelRc<SharedString>> {
    let words_list = m
        .get_vec()
        .iter()
        .map(|s| SharedString::from(*s))
        .collect::<Vec<SharedString>>();
    let mut chunks: Vec<ModelRc<SharedString>> = Vec::default();

    for chunk in words_list.chunks(chunk_size) {
        chunks.push(VecModel::from_slice(chunk));
    }

    VecModel::from_slice(&chunks)
}

pub fn from_bip39_model(model: ModelRc<ModelRc<SharedString>>) -> Result<Mnemonic, ZebraErrors> {
    let words = model
        .iter()
        .map(|chunk| {
            let str_vec = chunk.iter().map(|s| s.to_string()).collect::<Vec<String>>();

            str_vec.join(" ")
        })
        .collect::<Vec<String>>()
        .join(" ");
    let m = Mnemonic::mnemonic_to_entropy(Language::English, &words)?;

    Ok(m)
}

#[cfg(test)]
mod guard_tests {
    use crate::bip39::mnemonic::Language;

    use super::*;
    use rand;

    #[test]
    fn test_bip39_model_convert() {
        let mut rng = rand::thread_rng();

        let salt = "salt123";
        let m0 = Mnemonic::gen(&mut rng, 18, Language::English).unwrap();
        let model = gen_bip39_words(&m0, 3);
        let m1 = from_bip39_model(model).unwrap();

        assert_eq!(&m0.get_seed(salt), &m1.get_seed(salt));
    }
}
