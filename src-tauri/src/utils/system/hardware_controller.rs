use tauri::State;
use crate::utils::song::AudioState;

/// Set the volume
#[tauri::command]
pub fn set_volume(volume: f32, state: State<'_, AudioState>) -> Result<(), String> {
    let sink = state.sink.lock().unwrap();
    sink.set_volume(volume / 100.);
    Ok(())
}

/// Command to update the OS "Now Playing" UI
#[tauri::command]
pub fn update_system_metadata(title: String, artist: String, album: String, cover_url: String) {
    // Here you would normally interface with the 'souvlaki' or 'windows-metadata' crate
    // For now, this is the bridge for your system-api.ts
    println!("Updating System Media: {} by {}", title, artist);
}