//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)

#[derive(Debug, Default, Clone)]
pub enum Routers {
    #[default]
    Lock,
    LangChoose,
    Start,
    Mnemonic,
    Login,
    Privacy,
    SetupAccount,
    Home,
}
