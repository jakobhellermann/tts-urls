#![deny(missing_docs)]

//! This crate formats URLs to retrieve audio from TTS (text to speech) services like Google Translate or VoiceRSS.
//!
//! # Examples
//! Google Translate TTS:
//! ```
//! let url = google_translate_tts::google_tts::url("Hello, World!", "en");
//! assert_eq!(url, "https://translate.google.com/translate_tts?ie=UTF-8&q=Hello%2C%20World%21&tl=en&tk=418730.60457&client=webapp");
//! ```
//!
//! VoiceRSS:
//! ```
//! use google_translate_tts::voicerss::VoiceRSSOptions;
//! let key = "key";
//!
//! let url = VoiceRSSOptions::new()
//!     .language("de-de")
//!     .audio_format("32khz_16bit_stereo")
//!     .codec("mp3")
//!     .url(key, "Hallo Welt!");
//! assert_eq!(url, "http://api.voicerss.org/?key=key&hl=de-de&c=mp3&f=32khz_16bit_stereo&src=Hallo%20Welt%21");
//! ```

/// google translate tts
pub mod google_tts;
/// VoiceRSS [http://www.voicerss.org/](http://www.voicerss.org/)
pub mod voicerss;

// https://en.wikipedia.org/wiki/Percent-encoding#Types_of_URI_characters
// unreserved characters are 'a..zA..Z-_.~'
const ENCODE_SET: &'static percent_encoding::AsciiSet = &percent_encoding::NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'_')
    .remove(b'.')
    .remove(b'~');
