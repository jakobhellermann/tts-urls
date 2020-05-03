use std::borrow::Cow;

/// The languages supported by VoiceRSS
/// ([http://www.voicerss.org/api/documentation.aspx](http://www.voicerss.org/api/documentation.aspx))
pub const LANGUAGES: &[(&str, &str)] = &[
    ("ca-es", "Catalan"),
    ("zh-cn", "Chinese (China)"),
    ("zh-hk", "Chinese (Hong Kong)"),
    ("zh-tw", "Chinese (Taiwan)"),
    ("da-dk", "Danish"),
    ("nl-nl", "Dutch"),
    ("en-au", "English (Australia)"),
    ("en-ca", "English (Canada)"),
    ("en-gb", "English (Great Britain)"),
    ("en-in", "English (India)"),
    ("en-us", "English (United States)"),
    ("fi-fi", "Finnish"),
    ("fr-ca", "French (Canada)"),
    ("fr-fr", "French (France)"),
    ("de-de", "German"),
    ("it-it", "Italian"),
    ("ja-jp", "Japanese"),
    ("ko-kr", "Korean"),
    ("nb-no", "Norwegian"),
    ("pl-pl", "Polish"),
    ("pt-br", "Portuguese (Brazil)"),
    ("pt-pt", "Portuguese (Portugal)"),
    ("ru-ru", "Russian"),
    ("es-mx", "Spanish (Mexico)"),
    ("es-es", "Spanish (Spain)"),
    ("sv-se", "Swedish (Sweden)"),
];

/// The codecs supported by VoiceRSS
/// ([http://www.voicerss.org/api/documentation.aspx](http://www.voicerss.org/api/documentation.aspx))
pub const CODECS: &[&str] = &["MP3", "WAV", "AAC", "OGG", "CAF"];

/// The audio formats supported by VoiceRSS
/// ([http://www.voicerss.org/api/documentation.aspx](http://www.voicerss.org/api/documentation.aspx))
pub const AUDIO_FORMATS: &[&str] = &[
    "8khz_8bit_mono",
    "8khz_8bit_stereo",
    "8khz_16bit_mono",
    "8khz_16bit_stereo",
    "11khz_8bit_mono",
    "11khz_8bit_stereo",
    "11khz_16bit_mono",
    "11khz_16bit_stereo",
    "12khz_8bit_mono",
    "12khz_8bit_stereo",
    "12khz_16bit_mono",
    "12khz_16bit_stereo",
    "16khz_8bit_mono",
    "16khz_8bit_stereo",
    "16khz_16bit_mono",
    "16khz_16bit_stereo",
    "22khz_8bit_mono",
    "22khz_8bit_stereo",
    "22khz_16bit_mono",
    "22khz_16bit_stereo",
    "24khz_8bit_mono",
    "24khz_8bit_stereo",
    "24khz_16bit_mono",
    "24khz_16bit_stereo",
    "32khz_8bit_mono",
    "32khz_8bit_stereo",
    "32khz_16bit_mono",
    "32khz_16bit_stereo",
    "44khz_8bit_mono",
    "44khz_8bit_stereo",
    "44khz_16bit_mono",
    "44khz_16bit_stereo",
    "48khz_8bit_mono",
    "48khz_8bit_stereo",
    "48khz_16bit_mono",
    "48khz_16bit_stereo",
    "alaw_8khz_mono",
    "alaw_8khz_stereo",
    "alaw_11khz_mono",
    "alaw_11khz_stereo",
    "alaw_22khz_mono",
    "alaw_22khz_stereo",
    "alaw_44khz_mono",
    "alaw_44khz_stereo",
    "ulaw_8khz_mono",
    "ulaw_8khz_stereo",
    "ulaw_11khz_mono",
    "ulaw_11khz_stereo",
    "ulaw_22khz_mono",
    "ulaw_22khz_stereo",
    "ulaw_44khz_mono",
    "ulaw_44khz_stereo",
];

///
/// # Example usage:
/// ```rust
/// use tts_urls::voicerss::VoiceRSSOptions;
/// let key = "key";
///
/// let url = VoiceRSSOptions::new()
///     .language("de-de")
///     .audio_format("32khz_16bit_stereo")
///     .codec("mp3")
///     .url(key, "Hallo Welt!");
/// assert_eq!(url, "http://api.voicerss.org/?key=key&hl=de-de&c=mp3&f=32khz_16bit_stereo&src=Hallo%20Welt%21");
/// ```
#[derive(Default)]
pub struct VoiceRSSOptions {
    language: Option<Cow<'static, str>>,
    speed: Option<i8>,
    codec: Option<Cow<'static, str>>,
    audio_format: Option<Cow<'static, str>>,
    ssml: Option<bool>,
    base64: Option<bool>,
}

impl VoiceRSSOptions {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }

    /// see [VoiceRSS documentation](http://www.voicerss.org/api/documentation.aspx) for possible values
    pub fn language(&mut self, language: impl Into<Cow<'static, str>>) -> &mut Self {
        self.language = Some(language.into());
        self
    }

    /// The speech rate. Allows values from -10 to 10.
    pub fn speed(&mut self, speed: i8) -> &mut Self {
        assert!(
            speed >= -10 && speed <= 10,
            "speed should be between -10 and 10"
        );
        self.speed = Some(speed);
        self
    }

    /// see [VoiceRSS documentation](http://www.voicerss.org/api/documentation.aspx) for possible values
    pub fn codec(&mut self, codec: impl Into<Cow<'static, str>>) -> &mut Self {
        self.codec = Some(codec.into());
        self
    }

    /// see [www.voicerss.org/api/documentation.aspx](VoiceRSS documentation for possible values)
    pub fn audio_format(&mut self, audio_format: impl Into<Cow<'static, str>>) -> &mut Self {
        self.audio_format = Some(audio_format.into());
        self
    }

    /// Enable the SSML textual content format
    pub fn ssml(&mut self, ssml: bool) -> &mut Self {
        self.ssml = Some(ssml);
        self
    }

    /// makes the VoiceRSS api return the inline base64 `src` for an HTML <audio> element
    pub fn base64(&mut self, base64: bool) -> &mut Self {
        self.base64 = Some(base64);
        self
    }

    /// Returns the URL to the TTS audio for the given term and api key.
    pub fn url(&self, key: &str, text: &str) -> String {
        assert!(
            key.chars().all(char::is_alphanumeric),
            "key should be alphanumeric"
        );

        let language = self.language.as_deref().unwrap_or("en-us");
        let text = percent_encoding::utf8_percent_encode(text, crate::ENCODE_SET);

        let mut url = format!("http://api.voicerss.org/?key={}&hl={}", key, language);

        if let Some(speed) = self.speed {
            url.push_str(&format!("&r={}", speed));
        }
        if let Some(codec) = &self.codec {
            url.push_str(&format!("&c={}", codec));
        }
        if let Some(audio_format) = &self.audio_format {
            url.push_str(&format!("&f={}", audio_format));
        }
        if let Some(ssml) = &self.ssml {
            url.push_str(&format!("&sml={}", ssml));
        }

        url.push_str(&format!("&src={}", text));

        url
    }
}

/// Short version of [`VoiceRSSOptions::url`](struct.VoiceRSSOptions.html#method.url) with default options.
pub fn url(key: &str, text: &str) -> String {
    VoiceRSSOptions::default().url(key, text)
}

#[test]
#[should_panic]
fn invalid_speed() {
    VoiceRSSOptions::new().speed(11);
}

#[test]
#[should_panic]
fn invalid_key() {
    VoiceRSSOptions::new().url("/?;", "");
}

#[test]
fn unicode() {
    let url = VoiceRSSOptions::new()
        .language("ru-ru")
        .url("key", "Добрый день!");

    assert_eq!(
        url,
        "http://api.voicerss.org/?key=key&hl=ru-ru&src=%D0%94%D0%BE%D0%B1%D1%80%D1%8B%D0%B9%20%D0%B4%D0%B5%D0%BD%D1%8C%21"
    );
}
