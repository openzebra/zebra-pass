// -- Copyright (c) 2023 Rina Khasanshin
// -- Email: hicarus@yandex.ru
// -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::rc::Rc;

use rand;
use slint::{ModelRc, SharedString, VecModel};
use zebra_pass::bip39::mnemonic::Mnemonic;

slint::include_modules!();

// fn main() {
//     unimplemented!();
// }

fn main() -> Result<(), slint::PlatformError> {
    slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/locale/"));

    let app = AppWindow::new()?;

    let main = app.as_weak().unwrap();

    main.global::<Logic>().on_request_random_words(|| {
        let mut rng = rand::thread_rng();

        let m = Mnemonic::generate_mnemonic(&mut rng).unwrap();
        let words: Rc<VecModel<VecModel<SharedString>>> = Rc::new(VecModel::default());

        for chunk in m.get_list().chunks(3) {
            words.push(VecModel::from(chunk.into()));
        }

        ModelRc::from(words)
    });

    app.run()
}
