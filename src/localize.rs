//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    LanguageLoader,
};
use i18n_embed_fl::fl;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "i18n"] // path to the compiled localization resources
struct Localizations;

// #[macro_export]
// macro_rules! fl {
//     ($message_id:literal) => {{
//         i18n_embed_fl::fl!($crate::YOUR_STATIC_LOADER, $message_id)
//     }};
//
//     ($message_id:literal, $($args:expr),*) => {{
//         i18n_embed_fl::fl!($crate::YOUR_STATIC_LOADER, $message_id, $($args), *)
//     }};
// }

#[test]
fn test_i18n() {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    loader
        .load_languages(&Localizations, &[loader.fallback_language()])
        .unwrap();

    let value = fl!(loader, "network");

    assert_eq!(value, "Network");
}
