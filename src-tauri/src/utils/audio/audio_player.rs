use tauri::{App, AppHandle, Emitter, Runtime};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::sync::{Arc, Mutex, MutexGuard};
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

/// The Audio State
/// sink: The controller of the audio
/// total_duration: The total duration of the audio
pub struct AudioState {
    pub sink: Arc<Mutex<Sink>>,
    pub total_duration: Arc<Mutex<u64>>,
}

/// Init: Get the Audio State for operations
pub fn get_audio_state() -> (OutputStream, AudioState) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    (_stream, AudioState {
        sink: Arc::new(Mutex::new(sink)),
        total_duration: Arc::new(Mutex::new(0)),
    })
}

/// To start a thread to check if the playback is over
fn start_playback_swap(sink_clone: Arc<Mutex<Sink>>, handle: AppHandle) {
    std::thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(500));

            let sink = sink_clone.lock().unwrap();
            if sink.empty() {
                handle.emit("playback-finished", ()).unwrap();
                break;
            }
        }
    });
}

/// To start playing a song
#[tauri::command]
pub fn play_song(path: String, state: tauri::State<'_, AudioState>, app_handle: AppHandle) -> Result<(), String> {
    if cfg!(debug_assertions) {
        println!("Attempting to play: {}", path);
    }

    let file = File::open(&path).map_err(|e| e.to_string())?;
    let source = Decoder::new(BufReader::new(file)).map_err(|e| e.to_string())?;

    let sink = state.sink.lock().unwrap();
    let duration = source.total_duration().map(|d| d.as_secs()).unwrap_or(0);
    *state.total_duration.lock().unwrap() = duration;
    
    sink.stop();
    sink.append(source);
    sink.play();

    start_playback_swap(state.sink.clone(), app_handle.clone());

    Ok(())
}

/// Switch: Play/Pause
#[tauri::command]
pub fn toggle_playback(state: tauri::State<'_, AudioState>) -> Result<bool, String> {
    let sink = state.sink.lock().unwrap();
    if sink.is_paused() {
        sink.play();
        Ok(false)
    } else {
        sink.pause();
        Ok(true)
    }
}


/// The Audio Progress
/// position: Current position
/// duration: The total duration
#[derive(serde::Serialize)]
pub struct AudioProgress {
    pub position: u64,
    pub duration: u64,
}

#[tauri::command]
pub fn get_playback_progress(state: tauri::State<'_, AudioState>) -> AudioProgress {
    let sink = state.sink.lock().unwrap();
    let total_duration = state.total_duration.lock().unwrap();

    let position = sink.get_pos().as_secs();
    
    AudioProgress {
        position,
        duration: *total_duration,
    }
}
