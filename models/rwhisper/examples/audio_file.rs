use futures_util::StreamExt;
use rodio::Decoder;
use rwhisper::*;
use std::fs::File;
use std::io::BufReader;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create a new small whisper model
    let model = WhisperBuilder::default()
        .with_source(WhisperSource::SmallEn)
        .build()?;

    // Load audio from a file
    let file = BufReader::new(File::open("./models/rwhisper/examples/samples_jfk.wav").unwrap());
    // Decode that sound file into a source
    let audio = Decoder::new(file).unwrap();

    // Transcribe the source audio into text
    let mut text = model.transcribe(audio)?;

    // As the model transcribes the audio, print the text to the console
    while let Some(text) = text.next().await {
        print!("{}", text.text());
    }

    Ok(())
}
