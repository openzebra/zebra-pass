//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use crate::bip39::mnemonic::Mnemonic;
use rand;
use slint::{ModelRc, SharedString, VecModel};

pub fn gen_bip39_words(chunk_size: usize) -> ModelRc<ModelRc<SharedString>> {
    let mut rng = rand::thread_rng();

    let m = Mnemonic::generate_mnemonic(&mut rng).unwrap();
    let words_list = m.get_list().map(|s| SharedString::from(s));
    let mut chunks: Vec<ModelRc<SharedString>> = Vec::default();

    for chunk in words_list.chunks(chunk_size) {
        chunks.push(VecModel::from_slice(chunk));
    }

    VecModel::from_slice(&chunks)
}
