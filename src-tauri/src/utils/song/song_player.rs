use std::collections::HashMap;
use audiotags::Tag;
use base64::{engine::general_purpose, Engine as _};
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink, Source};
use std::fs::File;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{AppHandle, Emitter, State};
use lazy_static::lazy_static;

pub struct AudioState {
    sink: Arc<Mutex<Sink>>,
    is_paused: Arc<Mutex<AtomicBool>>,
}

#[derive(serde::Serialize, Clone)]
pub struct AudioMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub cover: Option<String>, // Base64 encoded image
    pub total_duration: f64,
}

pub fn get_audio_state() -> (OutputStream, AudioState) {
    let output_stream =
        OutputStreamBuilder::open_default_stream().expect("Failed to open default output stream.");
    let sink = Arc::new(Mutex::new(Sink::connect_new(&output_stream.mixer())));
    sink.lock().unwrap().set_volume(0.8);
    (
        output_stream,
        AudioState {
            sink,
            is_paused: Arc::new(Mutex::new(AtomicBool::new(false))),
        },
    )
}

/// Emit an event when the playback stops
pub fn spawn_playback_watcher(app_handle: AppHandle, state: &State<'_, AudioState>) {
    let sink_mutex = state.sink.clone();

    std::thread::spawn(move || {
        loop {
            // Sleep to avoid high CPU usage
            std::thread::sleep(Duration::from_millis(500));

            let sink = sink_mutex.lock().unwrap();
            // Check if the sink has finished all audio
            if sink.empty() {
                // Emit the event to the frontend
                app_handle.emit("playback-finished", {}).unwrap();
                break; // Exit the loop/thread once finished
            }
        }
    });
}

lazy_static! {
    // Cache maps file path -> AudioMetadata
    static ref METADATA_CACHE: Mutex<HashMap<String, AudioMetadata>> = Mutex::new(HashMap::new());
}

pub fn get_metadata(path: &String, total_duration: f64) -> AudioMetadata {
    // 1. Check if it's already in cache
    {
        let cache = METADATA_CACHE.lock().unwrap();
        if let Some(data) = cache.get(path) {
            return data.clone(); // Requires AudioMetadata to derive Clone
        }
    }

    // 2. If not found, do the heavy lifting
    let mut metadata = match Tag::new().read_from_path(&path) {
        Ok(tag) => {
            let cover_base64 = tag.album_cover().map(|cover| {
                let b64 = general_purpose::STANDARD.encode(cover.data);
                // Fix: use display format for mime_type, not Debug {:?}
                format!("data:{:?};base64,{}", cover.mime_type, b64)
            });
            AudioMetadata {
                title: tag.title().map(|s| s.to_string()),
                artist: tag.artist().map(|s| s.to_string()),
                album: tag.album().map(|a| a.title.to_string()),
                cover: cover_base64,
                total_duration,
            }
        }
        Err(_) => AudioMetadata {
            title: None,
            artist: None,
            album: None,
            cover: None,
            total_duration,
        },
    };

    // 3. Save to cache for next time
    let mut cache = METADATA_CACHE.lock().unwrap();
    cache.insert(path.clone(), metadata.clone());
    metadata.total_duration = total_duration;

    metadata
}

/// Get the metadata of some files.
#[tauri::command]
pub fn fetch_metadata(paths: Vec<String>) -> Result<Vec<AudioMetadata>, String> {
    Ok(paths.iter().map(|path| get_metadata(path, 0.)).collect())
}

/// Load the song with the path of the song file.
/// This function plays the song.
/// Return the total duration.
#[tauri::command]
pub fn load_song(
    path: String,
    state: State<'_, AudioState>,
    app_handle: AppHandle,
) -> Result<AudioMetadata, String> {
    let file = File::open(&path).map_err(|e| format!("[ERROR] Failed to open the file: {}", e))?;
    let source = Decoder::try_from(file).map_err(|e| format!("[ERROR] Failed to decode: {}", e))?;

    let total_duration = source.total_duration().unwrap().as_secs_f64();

    let sink = state.sink.lock().unwrap();
    sink.stop();
    sink.clear();
    sink.append(source);
    sink.play();

    *state.is_paused.lock().unwrap().get_mut() = false;

    spawn_playback_watcher(app_handle, &state);

    if cfg!(debug_assertions) {
        println!("[DEBUG] Succeeded to load the song...");
    }

    Ok(get_metadata(&path, total_duration))
}

/// Toggle playback.
/// Return a boolean meaning that if the playback is paused.
#[tauri::command]
pub fn toggle_playback(state: State<'_, AudioState>) -> Result<bool, String> {
    let sink = state.sink.lock().unwrap();

    let mut binding = state.is_paused.lock().unwrap();
    let origin_state = binding.get_mut();
    if *origin_state {
        sink.play();
    } else {
        sink.pause();
    }
    let new_state = sink.is_paused();

    if cfg!(debug_assertions) {
        println!(
            "Toggling playback state(is playing?): {} -> {}",
            origin_state, new_state
        );
    }

    *origin_state = new_state;

    Ok(new_state)
}

/// Fetch the song playback progress.
#[tauri::command]
pub fn fetch_progress(state: State<'_, AudioState>) -> Result<f64, String> {
    let sink = state.sink.lock().unwrap();
    let current_time = sink.get_pos().as_secs_f64();
    Ok(current_time)
}

/// Set the position
#[tauri::command]
pub fn set_position(time: f64, state: State<'_, AudioState>) -> Result<(), String> {
    let sink = state.sink.lock().unwrap();

    let seek_time = Duration::from_secs_f64(time);
    sink.try_seek(seek_time)
        .map_err(|e| format!("Failed to seek: {}", e))?;

    Ok(())
}

/// Set the volume
#[tauri::command]
pub fn set_volume(volume: f32, state: State<'_, AudioState>) -> Result<(), String> {
    let sink = state.sink.lock().unwrap();
    sink.set_volume(volume / 100.);
    Ok(())
}
