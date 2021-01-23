use tts_urls::voicerss::{Codec, VoiceRSSOptions};

fn main() {
    let key = std::env::var("VOICERSS_API_KEY").expect("no api key");

    let url = VoiceRSSOptions::new()
        .codec(Codec::MP3)
        .voice("Alice")
        .url(&key, "This is a test");

    println!("{}", url);
}
