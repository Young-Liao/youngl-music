mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (_stream, audio_state) = utils::song::get_audio_state();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_media::init())
        .manage(audio_state)
        .invoke_handler(tauri::generate_handler![
            utils::song::load_song,
            utils::song::stop_song,
            utils::song::toggle_playback,
            utils::song::fetch_progress,
            utils::song::set_position,
            utils::song::fetch_metadata,
            utils::system::set_volume,
            utils::system::update_system_metadata,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
