//! -- Copyright (c) 2023Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use regex::Regex;

use crate::errors::ZebraErrors;

pub fn is_valid_email(email: &str) -> Result<bool, ZebraErrors> {
    let email_regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$")
        .or(Err(ZebraErrors::RegexError))?;
    Ok(email_regex.is_match(email))
}

#[test]
fn test_email_validator() {
    assert!(is_valid_email("loh@sss.ru").unwrap());
    assert!(!is_valid_email("loh@ru").unwrap());
    assert!(!is_valid_email("lohru").unwrap());
}
