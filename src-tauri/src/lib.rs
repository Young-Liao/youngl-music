mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let audio_state = utils::audio::get_audio_state();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(audio_state.1)
        .invoke_handler(tauri::generate_handler![
            utils::audio::play_song,
            utils::audio::toggle_playback,
            utils::audio::get_playback_progress,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
