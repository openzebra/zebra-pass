// -- Copyright (c) 2023 Rina Khasanshin
// -- Email: hicarus@yandex.ru
// -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::{cell::RefCell, rc::Rc};

use rand;
use slint::SharedString;
use zebra_pass::{
    bip39::mnemonic::Mnemonic,
    core::{
        bip39::{self, from_bip39_model},
        core::Core,
    },
    errors::ZebraErrors,
};

slint::include_modules!();

fn handler(core: Rc<RefCell<Core>>) -> Result<(), slint::PlatformError> {
    slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/locale/"));

    let core_ref_state = core.clone();
    let state = &core_ref_state.borrow().state.clone();
    let app = AppWindow::new()?;
    let main_window = Rc::new(app.as_weak().unwrap());

    if !state.borrow().payload.inited {
        main_window.set_route(Routers::LangChoose);
    } else {
        main_window.set_route(Routers::Lock);
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
            let mut core = core_ref.borrow_mut();
            let sync = keys_logic_ref.global::<KeyChainLogic>().get_sync();
            let email = keys_logic_ref.global::<KeyChainLogic>().get_email();
            let password = keys_logic_ref.global::<KeyChainLogic>().get_password();
            let words_salt = keys_logic_ref.global::<KeyChainLogic>().get_words_salt();
            let words_model = keys_logic_ref.global::<KeyChainLogic>().get_random_words();
            let m = match from_bip39_model(words_model) {
                Ok(r) => r,
                Err(_) => {
                    return LogicResult {
                        error: "bip39 words are invalid".into(),
                        success: true,
                    }
                }
            };

            match core.init_data(sync, &email, &password, &words_salt, &m) {
                Ok(_) => LogicResult {
                    error: SharedString::default(),
                    success: true,
                },
                Err(_) => LogicResult {
                    // TODO: make more informative errors
                    error: "Cannot init data".into(),
                    success: false,
                },
            }
        });

    let lock_page_core_ref = core.clone();
    main_window
        .global::<KeyChainLogic>()
        .on_request_unlock(move |password| {
            let mut core = lock_page_core_ref.borrow_mut();

            match core.guard.try_unlock(&password.to_string().as_bytes()) {
                Ok(_) => LogicResult {
                    error: SharedString::default(),
                    success: true,
                },
                Err(_) => LogicResult {
                    // TODO: add more informative errors
                    error: "incorrect password".into(),
                    success: false,
                },
            }
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
