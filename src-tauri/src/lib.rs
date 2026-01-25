use crate::utils::system::PlayerControls;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State};

mod utils;

// --- 状态结构体：存储待处理的文件路径 ---
struct StartArgs(Mutex<Vec<String>>);

// --- 提取：统一的窗口唤醒逻辑 ---
fn wakeup_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.unminimize();
        let _ = window.show();
        let _ = window.set_focus(); // 或者是 set_focus()，取决于 tauri 版本
    }
}

// --- 提取：统一的参数处理与发送逻辑 ---
fn emit_file_args(app: &AppHandle, args: Vec<String>) {
    // 剔除第一个参数（程序名）
    let file_args = if args.len() > 1 {
        args[1..].to_vec()
    } else {
        Vec::new()
    };

    if !file_args.is_empty() {
        let _ = app.emit("open-file", file_args);
    }
}

// --- 提取：清理并格式化参数的逻辑 ---
fn parse_file_args(args: Vec<String>) -> Vec<String> {
    if args.len() > 1 {
        args[1..].to_vec()
    } else {
        Vec::new()
    }
}

// --- 指令：前端加载完成后调用，获取启动时的文件 ---
#[tauri::command]
fn get_start_args(state: State<StartArgs>) -> Vec<String> {
    let mut lock = state.0.lock().expect("Failed to lock start args");
    // std::mem::take 会留下一个空的 Vec，并将原有的数据返回
    std::mem::take(&mut *lock)
}

// Define a wrapper for our media controls to be stored in Tauri State
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (_stream, audio_state) = utils::song::get_audio_state();
    let initial_args = parse_file_args(std::env::args().collect());

    tauri::Builder::default()
        .manage(StartArgs(Mutex::new(initial_args)))
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            // 热启动：应用已在运行，直接通过 emit 推送给前端
            let file_args = parse_file_args(args);
            if !file_args.is_empty() {
                let _ = app.emit("open-file", file_args);
            }
            wakeup_window(app);
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
