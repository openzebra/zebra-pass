//! -- Copyright (c) 2023 Rina Khasanshin
//! -- Email: hicarus@yandex.ru
//! -- Licensed under the GNU General Public License Version 3.0 (GPL-3.0)
use std::borrow::Cow;

pub fn truncate_string<'a>(s: impl Into<Cow<'a, str>>, max_length: usize) -> Cow<'a, str> {
    let s = s.into();
    let len = s.len();

    if len <= max_length {
        s
    } else {
        let (start, end) = (
            s.get(..max_length / 2),
            s.get((len - max_length / 2)..).unwrap_or_default(),
        );
        let truncated = format!("{}...{}", start.unwrap_or_default(), end);
        Cow::Owned(truncated)
    }
}

#[cfg(test)]
mod utils_tests {
    use super::*;
    use rand::distributions::Alphanumeric;
    use rand::Rng;

    #[test]
    fn test_truncate_string_random() {
        let mut rng = rand::thread_rng();

        let num_tests = rng.gen_range(10..=100);

        for _ in 0..num_tests {
            let length = rng.gen_range(10..=100);

            let s: String = (&mut rng)
                .sample_iter(&Alphanumeric)
                .take(length)
                .map(char::from)
                .collect();

            let max_length = rng.gen_range(5..=length);

            let truncated = truncate_string(&s, max_length);

            if s.len() > max_length {
                assert!(truncated.contains("..."));

                let parts: Vec<&str> = truncated.split("...").collect();

                assert_eq!(parts.len(), 2);

                let start_len = parts[0].len();
                let end_len = parts[1].len();

                assert!(start_len == end_len);
            }
        }
    }
}
