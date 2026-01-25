use crate::utils::system::PlayerControls;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State};

mod utils;

// --- State Structure: Store pending file paths ---
struct StartArgs(Mutex<Vec<String>>);

// --- Wake up window logic (Desktop Only) ---
fn wakeup_window(app: &AppHandle) {
    // These window methods only exist on Desktop platforms
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.unminimize();
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

// --- Helper: Parse and clean arguments ---
fn parse_file_args(args: Vec<String>) -> Vec<String> {
    if args.len() > 1 {
        args[1..].to_vec()
    } else {
        Vec::new()
    }
}

// --- Command: Frontend calls this to get initial startup files ---
#[tauri::command]
fn get_start_args(state: State<StartArgs>) -> Vec<String> {
    let mut lock = state.0.lock().expect("Failed to lock start args");
    std::mem::take(&mut *lock)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 1. Audio state initialization
    let (_stream, audio_state) = utils::song::get_audio_state();

    // 2. CRITICAL FIX: Safe Argument Handling
    // std::env::args() triggers readlink, which causes SIGABRT on some Android emulators
    #[cfg(mobile)]
    let initial_args: Vec<String> = Vec::new();

    #[cfg(not(mobile))]
    let initial_args = parse_file_args(std::env::args().collect());

    // 3. Initialize Builder
    let mut builder = tauri::Builder::default();

    // 4. Add Single Instance Plugin (Desktop Only)
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            let file_args = parse_file_args(args);
            if !file_args.is_empty() {
                let _ = app.emit("open-file", file_args);
            }
            wakeup_window(app);
        }));
    }

    // 5. Build and Run the App
    builder
        .manage(StartArgs(Mutex::new(initial_args)))
        .manage(audio_state)
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // Windows specific AUMID setting
            #[cfg(target_os = "windows")]
            {
                let identifier = app.config().identifier.clone();
                let _ = utils::ta_win_util::set_aumid(&identifier);
            }

            // Manage media controls state
            app.manage(PlayerControls(Mutex::new(None)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_start_args,
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
