
#[cfg(not(feature = "raw"))]
use std::thread;
#[cfg(not(feature = "raw"))]
use rodio::{OutputStream,Decoder,Source};
#[cfg(not(feature = "raw"))]
use std::sync::Arc;
#[cfg(not(feature = "raw"))]
use rand::seq::IteratorRandom;






#[cfg(not(feature = "raw"))]
static FINISH_HIM: &[u8] = include_bytes!("../br/tuturu_1.mp3");
#[cfg(not(feature = "raw"))]
static FINISH_AHHH: &[u8] = include_bytes!("../br/anime-ahh.mp3");
#[cfg(not(feature = "raw"))]
static FINISH_UWU: &[u8] = include_bytes!("../br/youtube-uwuuuuu.mp3");
#[cfg(not(feature = "raw"))]
static FINISH_WOW: &[u8] = include_bytes!("../br/anime-wow-sound-effect.mp3");
#[cfg(not(feature = "raw"))]
pub fn play_finish_sound(volume:f32) {
    thread::spawn(move || {
    // Get the output stream handle to the default sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Create a Cursor to treat the byte slice as a file-like object
    let cursor = std::io::Cursor::new(FINISH_HIM);
    

    // Decode the MP3 data from the byte slice
    let source = Decoder::new(cursor).unwrap();

    let amplified_source = source.amplify(volume);
    // Play the sound
    let _ = stream_handle.play_raw(amplified_source.convert_samples());

    // Sleep to keep the main thread alive while the sound plays
    std::thread::sleep(std::time::Duration::from_secs(2));
    });
}
#[cfg(not(feature = "raw"))]
pub fn play_ahh_sound(volume: f32) {
    // Create a Vec<Vec<u8>> instead of Vec<&[u8]>
    let sounds = vec![
        FINISH_AHHH, // Convert to owned Vec<u8>
        FINISH_UWU,
        FINISH_WOW,
    ];

    // Wrap the sounds vector in Arc to make it 'static
    let sounds = Arc::new(sounds);

    thread::spawn(move || {
        // Get the output stream handle to the default sound device
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        // Choose a random sound from the sounds vector
        let random_sound: &[u8] = sounds.iter().choose(&mut rand::rng()).unwrap();

        // Create a Cursor to treat the byte slice as a file-like object
        let cursor = std::io::Cursor::new(random_sound);

        // Decode the MP3 data from the byte slice
        let source = Decoder::new(cursor).unwrap();

        let amplified_source = source.amplify(volume);

        // Play the sound
        let _ = stream_handle.play_raw(amplified_source.convert_samples());
        std::thread::sleep(std::time::Duration::from_secs(2));
    });
}