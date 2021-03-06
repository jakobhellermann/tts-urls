use std::borrow::Cow;

macro_rules! define_enum {
    ($name:ident with $($str:literal $($alias:literal)* $variant:ident)* ) => {
        #[allow(missing_docs)]
        #[derive(Debug, PartialEq, Eq, Copy, Clone)]
        pub enum $name {
            $($variant),*
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                let s = match self {
                    $( $name::$variant => $str, )*
                };
                f.write_str(s)
            }
        }

        impl std::str::FromStr for $name {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let val = match s.to_lowercase().as_str() {
                    $($str $(| $alias)* => $name::$variant,)*
                    _ => return Err(()),
                };
                Ok(val)
            }
        }
    };
}

define_enum!(Codec with
    "mp3" MP3
    "wav" WAV
    "aac" AAC
    "ogg" OGG
    "caf" CAF
);
impl Default for Codec {
    fn default() -> Self {
        Codec::MP3
    }
}

define_enum!(Language with
    "ar-eg" ArabicEgypt
    "ar-sa" ArabicSaudiArabia
    "bg-bg" "bg" Bulgarian
    "ca-es" Catalan
    "zh-cn" Chinese
    "zh-hk" ChineseHongKong
    "zh-tw" ChineseTaiwan
    "hr-hr" Croatian
    "cs-cz" Czech
    "da-dk" Danish
    "nl-be" DutchBelgium
    "nl-nl" DutchNetherlands
    "en-au" EnglishAustralia
    "en-ca" EnglishCanada
    "en-gb" EnglishGreatBritain
    "en-in" EnglishIndia
    "en-ie" EnglishIreland
    "en-us" EnglishUnitedStates
    "fi-fi" "fi" Finnish
    "fr-ca" FrenchCanada
    "fr-fr" "fr" French
    "fr-ch" FrenchSwitzerland
    "de-de" "de" German
    "de-at" GermanAustria
    "de-ch" GermanSwitzerland
    "el-gr" Greek
    "he-il" Hebrew
    "hi-in" Hindi
    "hu-hu" "hu" Hungarian
    "id-id" "id" Indonesian
    "it-it" "it" Italian
    "ja-jp" Japanese
    "ko-kr" Korean
    "ms-my" Malay
    "nb-no" "no" Norwegian
    "pl-pl" "pl" Polish
    "pt-pt" Portuguese
    "pt-br" PortugueseBrazil
    "ro-ro" "ro" Romanian
    "ru-ru" "ru" Russian
    "sk-sk" "sk" Slovak
    "sl-si" Slovenian
    "es-es" "es" Spanish
    "es-mx" SpanishMexico
    "sv-se" Swedish
    "ta-in" Tamil
    "th-th" "th" Thai
    "tr-tr" "tr" Turkish
    "vi-vn" Vietnamese
);
impl Default for Language {
    fn default() -> Self {
        Language::EnglishUnitedStates
    }
}

/// The audio formats supported by VoiceRSS
///
/// <http://www.voicerss.org/api/>
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
/// use tts_urls::voicerss::{VoiceRSSOptions, Language, Codec};
/// let key = "key";
///
/// let url = VoiceRSSOptions::new()
///     .language(Language::German)
///     .audio_format("32khz_16bit_stereo")
///     .codec(Codec::MP3)
///     .url(key, "Hallo Welt!");
/// assert_eq!(url, "http://api.voicerss.org/?key=key&hl=de-de&c=MP3&f=32khz_16bit_stereo&src=Hallo%20Welt%21");
/// ```
#[derive(Default)]
pub struct VoiceRSSOptions {
    language: Option<Language>,
    voice: Option<Cow<'static, str>>,
    speed: Option<i8>,
    codec: Option<Codec>,
    audio_format: Option<Cow<'static, str>>,
    ssml: Option<bool>,
    base64: Option<bool>,
}

impl VoiceRSSOptions {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }

    /// see [VoiceRSS documentation](http://www.voicerss.org/api/) for possible values
    pub fn language(&mut self, language: Language) -> &mut Self {
        self.language = Some(language);
        self
    }

    /// see [VoiceRSS documentation](http://www.voicerss.org/api/) for possible values
    pub fn voice(&mut self, voice: impl Into<Cow<'static, str>>) -> &mut Self {
        self.voice = Some(voice.into());
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

    /// see [VoiceRSS documentation](http://www.voicerss.org/api/) for possible values
    pub fn codec(&mut self, codec: Codec) -> &mut Self {
        self.codec = Some(codec);
        self
    }

    /// see [www.voicerss.org/api/documentation.aspx](VoiceRSS documentation for possible values)
    pub fn audio_format(&mut self, audio_format: impl Into<Cow<'static, str>>) -> &mut Self {
        let format = audio_format.into();

        assert!(AUDIO_FORMATS.iter().any(|&f| f == format));

        self.audio_format = Some(format);
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

        let language = self.language.unwrap_or_default();
        let text = percent_encoding::utf8_percent_encode(text, crate::ENCODE_SET);

        let mut url = format!("http://api.voicerss.org/?key={}&hl={}", key, language);

        if let Some(voice) = &self.voice {
            url.push_str(&format!("&v={}", voice));
        }
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
        .language(Language::Russian)
        .url("key", "Добрый день!");

    assert_eq!(
        url,
        "http://api.voicerss.org/?key=key&hl=ru-ru&src=%D0%94%D0%BE%D0%B1%D1%80%D1%8B%D0%B9%20%D0%B4%D0%B5%D0%BD%D1%8C%21"
    );
}
#[test]
fn language_parse() {
    assert_eq!(Language::ArabicEgypt, "ar-eg".parse().unwrap());
}

#[test]
fn language_alias() {
    assert_eq!(Language::German, "de".parse().unwrap());
}
