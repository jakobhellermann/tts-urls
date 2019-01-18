// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

//! This crate formats URLs to retrieve audio from the TTS (text to speech) service for Google Translate.
//!
//! # Disclaimer
//!
//! This crate is not affiliated with Google. The mentioning of Google's trademarks in the crate name is for informational purposes only.
//!
//! # Caution
//!
//! The URL format may change at any time. This crate may not work in the future.
//!
//! # Examples
//!
//! ```
//! let url = google_translate_tts::url("Hello, World!", "en");
//! assert_eq!(url, "https://translate.google.com/translate_tts?ie=UTF-8&q=Hello, World!&tl=en&tk=418730.60457&client=webapp");
//! ```

#![deny(missing_docs)]

mod encode_set {
    use percent_encoding::define_encode_set;

    define_encode_set! {
        pub ENCODE_SET = [percent_encoding::SIMPLE_ENCODE_SET] | {'&'}
    }
}

fn compute_checksum(term: &str) -> (u32, u32) {
    let mut checksum: u32 = 429955;
    for &byte in term.as_bytes() {
        checksum = checksum.wrapping_add(byte as _);
        checksum = checksum.wrapping_add(checksum << 10);
        checksum ^= checksum >> 6;
    }
    checksum = checksum.wrapping_add(checksum << 3);
    checksum ^= checksum >> 11;
    checksum = checksum.wrapping_add(checksum << 15);
    checksum ^= 3864579582;
    checksum %= 1_000_000;
    (checksum, checksum ^ 429955)
}

/// Returns the URL to the TTS audio for the given term and language.
///
/// # Examples
///
/// ```
/// let url = google_translate_tts::url("Hello, World!", "en");
/// assert_eq!(url, "https://translate.google.com/translate_tts?ie=UTF-8&q=Hello, World!&tl=en&tk=418730.60457&client=webapp");
/// ```
#[must_use]
pub fn url(term: &str, language: &str) -> String {
    let checksum = compute_checksum(term);
    format!(
        "https://translate.google.com/translate_tts?ie=UTF-8&q={}&tl={}&tk={}.{}&client=webapp",
        percent_encoding::utf8_percent_encode(term, encode_set::ENCODE_SET),
        language,
        checksum.0,
        checksum.1
    )
}

/// Returns the URL to the TTS audio for the given term, language and speed.
///
/// # Examples
///
/// ```
/// let url = google_translate_tts::url_with_speed("Hello, World!", "en", 0.24);
/// assert_eq!(url, "https://translate.google.com/translate_tts?ie=UTF-8&q=Hello, World!&tl=en&tk=418730.60457&client=webapp&ttsspeed=0.24");
/// ```
#[must_use]
pub fn url_with_speed(term: &str, language: &str, speed: f32) -> String {
    let checksum = compute_checksum(term);
    format!(
        "https://translate.google.com/translate_tts?ie=UTF-8&q={}&tl={}&tk={}.{}&client=webapp&ttsspeed={}",
        percent_encoding::utf8_percent_encode(term, encode_set::ENCODE_SET),
        language,
        checksum.0,
        checksum.1,
        speed
    )
}

#[test]
fn test1() {
    assert_eq!(
        url("Добрый день!", "ru"),
        "https://translate.google.com/translate_tts?ie=UTF-8&q=%D0%94%D0%BE%D0%B1%D1%80%D1%8B%D0%B9 %D0%B4%D0%B5%D0%BD%D1%8C!&tl=ru&tk=33233.396882&client=webapp"
    );
}

#[test]
fn test2() {
    assert_eq!(
        url_with_speed("Добрый день!", "ru", 0.24),
        "https://translate.google.com/translate_tts?ie=UTF-8&q=%D0%94%D0%BE%D0%B1%D1%80%D1%8B%D0%B9 %D0%B4%D0%B5%D0%BD%D1%8C!&tl=ru&tk=33233.396882&client=webapp&ttsspeed=0.24"
    );
}

#[test]
fn test3() {
    assert_eq!(
        url("\n&", "ru"),
        "https://translate.google.com/translate_tts?ie=UTF-8&q=%0A%26&tl=ru&tk=589697.946178&client=webapp"
    );
}
