//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
extern crate rust_i18n;

use rust_i18n::i18n;
// use zebra_ui::theme;

i18n!("locales", fallback = "en");

// enum State {
//     Loader(Box<Loader>),
//     App(App),
// }
//
// pub struct GUI {
//     state: State,
// }
//
// pub enum Message {
//     CtrlC,
//     Load(Box<loader::Message>),
//     Run(Box<app::Message>),
//     Event(iced_native::Event),
// }
//
// async fn ctrl_c() -> Result<(), ()> {
//     if let Err(e) = tokio::signal::ctrl_c().await {
//         error!("{}", e);
//     };
//     info!("Signal received, exiting");
//     Ok(())
// }

fn main() {
    println!("Helloiiiii world!");
}
