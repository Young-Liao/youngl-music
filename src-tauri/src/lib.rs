mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (_stream, audio_state) = utils::song::get_audio_state();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(audio_state)
        .invoke_handler(tauri::generate_handler![
            utils::song::load_song,
            utils::song::toggle_playback,
            utils::song::fetch_progress,
            utils::song::set_position,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
