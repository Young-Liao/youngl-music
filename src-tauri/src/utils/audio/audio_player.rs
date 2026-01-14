use kira::backend::cpal::CpalBackend;
use kira::{sound::static_sound::{StaticSoundData, StaticSoundHandle}, AudioManager, AudioManagerSettings, Tween};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, State};

/// The Audio State
pub struct AudioState {
    // Kira's manager handles the audio device connection
    pub manager: Arc<Mutex<AudioManager<CpalBackend>>>,
    // The handle allows us to control the sound currently playing (seek, pause, stop)
    pub player_handle: Arc<Mutex<Option<StaticSoundHandle>>>,
    pub total_duration: Arc<Mutex<f64>>,
}

/// Init: Set up the Kira Manager
pub fn get_audio_state() -> AudioState {
    let manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default())
        .expect("Failed to initialize Kira AudioManager");

    AudioState {
        manager: Arc::new(Mutex::new(manager)),
        player_handle: Arc::new(Mutex::new(None)),
        total_duration: Arc::new(Mutex::new(0.0)),
    }
}

/// Thread to monitor playback completion
fn start_playback_monitor(state: Arc<Mutex<Option<StaticSoundHandle>>>, handle: AppHandle) {
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(500));

            let lock = state.lock().unwrap();
            if let Some(sound_handle) = &*lock {
                // Kira handles play states explicitly
                if sound_handle.state() == kira::sound::PlaybackState::Stopped {
                    handle.emit("playback-finished", ()).unwrap();
                    break;
                }
            } else {
                break;
            }
        }
    });
}

#[tauri::command]
pub fn play_song(path: String, state: tauri::State<'_, AudioState>, app_handle: AppHandle) -> Result<(), String> {
    let mut manager = state.manager.lock().unwrap();

    // Load the flac file into a static, seekable buffer
    let sound_data = StaticSoundData::from_file(&path)
        .map_err(|e| format!("Failed to load sound: {}", e))?;

    let duration = sound_data.duration().as_secs_f64();
    *state.total_duration.lock().unwrap() = duration;

    let mut handle_lock = state.player_handle.lock().unwrap();

    // Stop previous handle if it exists
    if let Some(mut old_handle) = handle_lock.take() {
        let _ = old_handle.stop(Tween::default());
    }

    // Play (this creates the handle in a paused state)
    let handle = manager.play(sound_data).map_err(|e| e.to_string())?;
    *handle_lock = Some(handle);

    // Start your existing monitor thread
    start_playback_monitor(state.player_handle.clone(), app_handle);

    Ok(())
}

#[tauri::command]
pub fn toggle_playback(state: State<'_, AudioState>) -> Result<bool, String> {
    let mut handle_lock = state.player_handle.lock().unwrap();
    if let Some(handle) = handle_lock.as_mut() {
        if handle.state() == kira::sound::PlaybackState::Playing {
            // Optional: Add a small fade out for "Advanced UI" feel
            handle.pause(Tween {
                duration: std::time::Duration::from_millis(100),
                ..Default::default()
            });
            Ok(true) // Is Paused
        } else {
            handle.resume(Tween {
                duration: std::time::Duration::from_millis(100),
                ..Default::default()
            });
            Ok(false) // Is Playing
        }
    } else {
        Err("No audio loaded".to_string())
    }
}

#[derive(serde::Serialize)]
pub struct AudioProgress {
    pub position: f64,
    pub duration: f64,
}

#[tauri::command]
pub fn get_playback_progress(state: State<'_, AudioState>) -> AudioProgress {
    let handle_lock = state.player_handle.lock().unwrap();
    let duration = *state.total_duration.lock().unwrap();

    let position = if let Some(handle) = &*handle_lock {
        handle.position()
    } else {
        0.0
    };

    AudioProgress { position, duration }
}

#[tauri::command]
pub fn seek_audio(time: f64, state: State<'_, AudioState>) -> Result<(), String> {
    let mut handle_lock = state.player_handle.lock().unwrap();
    if let Some(handle) = handle_lock.as_mut() {
        // Kira seeking is native and works on samples directly
        handle.seek_to(time);
        Ok(())
    } else {
        Err("No active audio to seek".to_string())
    }
}
