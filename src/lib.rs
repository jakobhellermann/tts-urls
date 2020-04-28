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
//! let url = google_translate_tts::google_tts::url("Hello, World!", "en");
//! assert_eq!(url, "https://translate.google.com/translate_tts?ie=UTF-8&q=Hello%2C%20World%21&tl=en&tk=418730.60457&client=webapp");
//! ```

pub mod google_tts;
