// -- Copyright (c) 2023 Rina Khasanshin
// -- Email: hicarus@yandex.ru
// -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use std::rc::Rc;

use zebra_pass::{
    core::{bip39, core::Core},
    errors::ZebraErrors,
};

slint::include_modules!();

fn handler(core: Rc<Core>) -> Result<(), slint::PlatformError> {
    slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/locale/"));

    let state = core.state.borrow();
    let app = AppWindow::new()?;
    let main_window = Rc::new(app.as_weak().unwrap());

    if !state.payload.inited {
        main_window.set_route(Routers::LangChoose);
    }

    main_window
        .global::<KeyChainLogic>()
        .on_request_random_words(|| bip39::gen_bip39_words(3));

    let keys_logic_ref = main_window.clone();
    main_window
        .global::<KeyChainLogic>()
        .on_request_create_account(move || {
            let sync = keys_logic_ref.global::<KeyChainLogic>().get_sync();
            let email = keys_logic_ref.global::<KeyChainLogic>().get_email();
            let password = keys_logic_ref.global::<KeyChainLogic>().get_password();
            let words_salt = keys_logic_ref.global::<KeyChainLogic>().get_words_salt();
            let words = keys_logic_ref.global::<KeyChainLogic>().get_random_words();

            dbg!(sync, email, password, words_salt, words);

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

    handler(Rc::new(core))
}
