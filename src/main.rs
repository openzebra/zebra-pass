// -- Copyright (c) 2023 Rina Khasanshin
// -- Email: hicarus@yandex.ru
// -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use rand;
use slint::{ModelRc, SharedString, VecModel};
use zebra_pass::bip39::mnemonic::Mnemonic;

slint::include_modules!();

// fn main() {
//     let mut rng = rand::thread_rng();
//
//     let m = Mnemonic::generate_mnemonic(&mut rng).unwrap();
//     let words_list = m.get_list().map(|s| SharedString::from(s));
//     let mut chunks: Vec<ModelRc<SharedString>> = Vec::default();
//
//     for chunk in words_list.chunks(3) {
//         chunks.push(VecModel::from_slice(chunk));
//     }
//
//     let shared_words = VecModel::from_slice(&chunks);
// }

fn main() -> Result<(), slint::PlatformError> {
    slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/locale/"));

    let app = AppWindow::new()?;
    let main_window = app.as_weak().unwrap();

    main_window.global::<Logic>().on_request_random_words(|| {
        let mut rng = rand::thread_rng();

        let m = Mnemonic::generate_mnemonic(&mut rng).unwrap();
        let words_list = m.get_list().map(|s| SharedString::from(s));
        let mut chunks: Vec<ModelRc<SharedString>> = Vec::default();

        for chunk in words_list.chunks(3) {
            chunks.push(VecModel::from_slice(chunk));
        }

        VecModel::from_slice(&chunks)
    });

    app.run()
}
