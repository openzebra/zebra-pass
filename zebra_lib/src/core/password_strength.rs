//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use regex::Regex;

use crate::errors::ZebraErrors;

const IDIAL: f32 = 2500.0;
pub const MIN_PASSWORD_SIZE: usize = 10;

pub fn password_strength(password: &str) -> Result<u8, ZebraErrors> {
    let mut strength = 0.0;
    let password_length = password.len();

    let password_length = password_length as f32;

    strength += password_length;

    let calculate_score = |regex: &str, score_per_match: f32| -> Result<f32, ZebraErrors> {
        let re = Regex::new(regex).or(Err(ZebraErrors::RegexError))?;
        let count = re.find_iter(password).count() as f32;

        Ok(count * score_per_match)
    };

    strength += calculate_score(r"[A-Z]", 30.75)?;
    strength += calculate_score(r"[a-z]", 30.75)?;
    strength += calculate_score(r"\d", 30.75)?;
    strength += calculate_score(r"[^\w\s]", 90.75)?;

    Ok((strength / IDIAL * 100.0).min(100.0) as u8)
}

#[test]
fn test_password_strengh() {
    assert_eq!(password_strength("NS82ng)_Fn342").unwrap(), 17);
    assert_eq!(password_strength("testing2%55@3AJGU").unwrap(), 26);
    assert_eq!(password_strength("bXdN*6k4d55LdyT5ULj8awiW7^M9Z9CDhzx2LGNNpzPexrVCXRQbLK$BfmSpwf!GtH2ETCkbu$dPZv#ogEB#i6&TQiwc2*dWJzLom4oTsrjYCVED3WAFbdc$Kr!sk46y").unwrap(), 100);
}
