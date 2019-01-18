# Google Translate TTS

This crate formats URLs to retrieve audio from the TTS (text to speech) service for Google Translate.

## Disclaimer

This crate is not affiliated with Google. The mentioning of Google's trademarks in the crate name is for informational purposes only.

## Caution

The URL format may change at any time. This crate may not work in the future.

## Examples

```rust
let url = google_translate_tts::url("Hello, World!", "en");
assert_eq!(url, "https://translate.google.com/translate_tts?ie=UTF-8&q=Hello, World!&tl=en&tk=418730.60457&client=webapp");
```
