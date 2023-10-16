// -- Copyright (c) 2023 Rina Khasanshin
// -- Email: hicarus@yandex.ru
// -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::{cell::RefCell, rc::Rc};

use rand;
use zebra_pass::{
    bip39::mnemonic::Mnemonic,
    core::{
        bip39::{self, from_bip39_model},
        core::Core,
    },
    errors::ZebraErrors,
    records::records::Records,
};

slint::include_modules!();

fn handler(core: Rc<RefCell<Core>>) -> Result<(), slint::PlatformError> {
    slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/locale/"));

    let core_ref_state = core.clone();
    let state = &core_ref_state.borrow().state;
    let app = AppWindow::new()?;
    let main_window = Rc::new(app.as_weak().unwrap());

    if !state.borrow().payload.inited {
        main_window.set_route(Routers::LangChoose);
    }

    main_window
        .global::<KeyChainLogic>()
        .on_request_random_words(|| {
            let mut rng = rand::thread_rng();
            // TODO: make a error hanlder.
            let m = Mnemonic::generate_mnemonic(&mut rng).unwrap();
            bip39::gen_bip39_words(&m, 3)
        });

    let keys_logic_ref = main_window.clone();
    let core_ref = core.clone();
    main_window
        .global::<KeyChainLogic>()
        .on_request_create_account(move || {
            let sync = keys_logic_ref.global::<KeyChainLogic>().get_sync();
            let email = keys_logic_ref.global::<KeyChainLogic>().get_email();
            let password = keys_logic_ref.global::<KeyChainLogic>().get_password();
            let words_salt = keys_logic_ref.global::<KeyChainLogic>().get_words_salt();
            let words_model = keys_logic_ref.global::<KeyChainLogic>().get_random_words();
            // TODO: make error hanlder!
            let words = from_bip39_model(words_model).unwrap().get();
            let data: Vec<Records> = Vec::new();
            let mut mut_core = core_ref.borrow_mut();

            // TODO: make error hanlder!
            mut_core
                .guard
                .bip39_cipher_from_password::<Vec<Records>>(
                    &password.as_bytes(),
                    &words,
                    &words_salt,
                    data,
                )
                .unwrap();
            let mut mut_state = mut_core.state.borrow_mut();

            mut_state.payload.email = Some(email.to_string());
            // TODO: make error hanlder!
            mut_state.payload.address = mut_core.guard.get_address().unwrap();
            mut_state.payload.inited = true;
            mut_state.payload.restoreble = !email.is_empty();
            mut_state.payload.server_sync = sync;

            [].into()
        });

    app.run()
}

fn error_handler(error: ZebraErrors) -> Result<(), slint::PlatformError> {
    dbg!(error);
    // TODO: Show error window!
    Ok(())
}

// fn main() {
//     handler().unwrap();
// }

fn main() -> Result<(), slint::PlatformError> {
    let core = match Core::new() {
        Ok(c) => c,
        Err(e) => {
            return error_handler(e);
        }
    };

    match core.sync() {
        Ok(_) => {}
        Err(e) => {
            return error_handler(e);
        }
    }

    handler(Rc::new(RefCell::new(core)))
}
