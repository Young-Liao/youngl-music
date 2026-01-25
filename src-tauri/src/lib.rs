use crate::utils::system::PlayerControls;
use std::sync::Mutex;
use tauri::Manager;

mod utils;

// Define a wrapper for our media controls to be stored in Tauri State
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (_stream, audio_state) = utils::song::get_audio_state();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            // 当第二个实例启动时，args 会包含新的文件路径
            // 将路径发送给前端播放
            app.emit("open-file", args).unwrap();

            // 顺便把窗口拉到最前面
            let _ = app.get_webview_window("main").map(|w| w.set_focus());
        }))
        .setup(|app| {
            #[cfg(target_os = "windows")]
            {
                let identifier = app.config().identifier.clone();
                // 强制 Windows 将此进程与标识符关联
                let _ = utils::ta_win_util::set_aumid(&identifier);
            }
            // 仅注册空状态，不执行任何 souvlaki 初始化
            app.manage(PlayerControls(Mutex::new(None)));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(audio_state)
        .invoke_handler(tauri::generate_handler![
            utils::song::load_song,
            utils::song::stop_song,
            utils::song::toggle_playback,
            utils::song::fetch_progress,
            utils::song::set_position,
            utils::song::fetch_metadata,
            utils::song::load_lyrics_from_lrc,
            utils::song::load_lyrics_from_song,
            utils::system::set_volume,
            utils::system::update_system_metadata,
            utils::system::update_system_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
