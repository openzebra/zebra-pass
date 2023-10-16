// -- Copyright (c) 2023 Rina Khasanshin
// -- Email: hicarus@yandex.ru
// -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

use zebra_pass::{
    core::{bip39, core::Core},
    errors::ZebraErrors,
};

slint::include_modules!();

fn handler(core: Core) -> Result<(), slint::PlatformError> {
    slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/locale/"));
    let app = AppWindow::new()?;
    let main_window = app.as_weak().unwrap();
    let weak_keys_logic = Rc::new(app.as_weak().unwrap().global::<KeyChainLogic>());

    main_window
        .global::<KeyChainLogic>()
        .on_request_random_words(|| bip39::gen_bip39_words(3));

    main_window
        .global::<KeyChainLogic>()
        .on_request_create_account(|| {
            let sync = weak_logic.get_sync();

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

    handler(core)
}
