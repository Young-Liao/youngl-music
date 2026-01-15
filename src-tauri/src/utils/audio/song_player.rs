use std::fs::File;
use std::path::Path;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use tauri::{AppHandle, State};

pub struct AudioState {
    pub total_duration: Arc<Mutex<f64>>,
    pub is_ready: Arc<AtomicBool>,   // True once the file is probed
    pub is_playing: Arc<AtomicBool>, // True if thread is currently pushing samples
    pub cvar: Arc<(Mutex<bool>, Condvar)>,
}

pub fn get_audio_state() -> AudioState {
    AudioState {
        total_duration: Arc::new(Mutex::new(0.0)),
        is_ready: Arc::new(AtomicBool::new(false)),
        is_playing: Arc::new(AtomicBool::new(false)),
        cvar: Arc::new((Mutex::new(false), Condvar::new())),
    }
}

#[tauri::command]
pub fn load_song(path: String, state: State<'_, AudioState>) -> Result<f64, String> {
    // 1. Open the media source (the file)
    let src = File::open(Path::new(&path)).map_err(|e| e.to_string())?;

    // 2. Create the media source stream
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    // 3. Create a hint to help the probe (extension)
    let mut hint = Hint::new();
    if let Some(ext) = Path::new(&path).extension().and_then(|s| s.to_str()) {
        hint.with_extension(ext);
    }

    // 4. Probe the media source
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())
        .map_err(|e| e.to_string())?;

    let format = probed.format;

    // 5. Find the first valid audio track
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or("No supported audio track found")?;

    // 6. Calculate Duration accurately
    let mut duration_secs = 0.0;
    if let Some(n_frames) = track.codec_params.n_frames {
        if let Some(tb) = track.codec_params.time_base {
            duration_secs = n_frames as f64 * (tb.numer as f64 / tb.denom as f64);
        }
    }

    // 7. Update State
    *state.total_duration.lock().unwrap() = duration_secs;
    state.is_ready.store(true, Ordering::SeqCst);

    if cfg!(debug_assertions) {
        println!("Loaded: {} | Duration: {:.2}s", path, duration_secs);
    }

    Ok(duration_secs)
}
#[tauri::command]
pub fn toggle_playback(state: State<'_, AudioState>) -> Result<bool, String> {
    // 🛡️ Safety: Don't toggle if the file isn't loaded
    if !state.is_ready.load(Ordering::SeqCst) {
        return Err("No audio loaded".to_string());
    }

    let was_playing = state.is_playing.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |p| Some(!p)).unwrap();
    let is_now_playing = !was_playing;

    let (lock, cvar) = &*state.cvar;
    let mut paused = lock.lock().unwrap();
    *paused = !is_now_playing;

    if is_now_playing {
        cvar.notify_one();
    }

    Ok(is_now_playing)
}
