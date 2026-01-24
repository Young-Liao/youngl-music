use std::fs;
use std::sync::Mutex;
use std::time::Duration;
use base64::Engine;
use base64::engine::general_purpose;
use tauri::{AppHandle, Emitter, Manager, State, Url};
use souvlaki::{MediaControlEvent, MediaControls, MediaMetadata, PlatformConfig, MediaPlayback, MediaPosition};
use crate::utils::song::AudioState;

/// Set the volume
#[tauri::command]
pub fn set_volume(volume: f32, state: State<'_, AudioState>) -> Result<(), String> {
    let sink = state.sink.lock().unwrap();
    sink.set_volume(volume / 100.);
    Ok(())
}

// 状态结构体：里面存的是 Option
pub struct PlayerControls(pub Mutex<Option<MediaControls>>);

#[tauri::command]
pub async fn update_system_metadata(
    app: AppHandle,
    state: State<'_, PlayerControls>,
    title: String,
    artist: String,
    album: String,
    cover: Option<String>,
) -> Result<(), String> {
    let mut control_slot = state.0.lock().unwrap();

    // 1. 初始化 (保持不变)
    if control_slot.is_none() {
        let window = app.get_webview_window("main")
            .ok_or_else(|| "Main window not found".to_string())?;

        #[cfg(target_os = "windows")]
        let hwnd = Some(window.hwnd().map_err(|e| e.to_string())?.0 as *mut std::ffi::c_void);
        #[cfg(not(target_os = "windows"))]
        let hwnd = None;

        let config = PlatformConfig {
            dbus_name: "com.young.youngl-music", // 每次改代码建议微调名字，强制 Windows 刷新
            display_name: "YoungL Music",
            hwnd,
        };

        let mut c = MediaControls::new(config).map_err(|e| format!("{:?}", e))?;
        c.set_playback(MediaPlayback::Playing { progress: None }).ok();

        let handle = app.clone();
        c.attach(move |event| {
            let signal = match event {
                MediaControlEvent::Toggle | MediaControlEvent::Play | MediaControlEvent::Pause => "Toggle",
                MediaControlEvent::Next => "Next",
                MediaControlEvent::Previous => "Previous",
                _ => "Unknown",
            };
            let _ = handle.emit("system-media-event", signal);
        }).map_err(|e| format!("{:?}", e))?;

        *control_slot = Some(c);
    }

    // 2. 转换 Base64 为本地文件路径
    let mut local_cover_url = None;
    if let Some(base64_str) = cover {
        let base64_data = base64_str.split(',').last().unwrap_or(&base64_str);

        if let Ok(bytes) = general_purpose::STANDARD.decode(base64_data) {
            // 【优化】尝试使用系统临时目录，路径更短，减少超限风险
            let temp_path = std::env::temp_dir().join("yt_music_cover.jpg");

            // 确保旧文件被删除并写入新文件
            let _ = fs::remove_file(&temp_path);
            if fs::write(&temp_path, bytes).is_ok() {
                local_cover_url = Some(String::from(temp_path.to_str().unwrap().to_string()));
            }
        }
    }

    // 3. 提交元数据
    if let Some(ref mut controls) = *control_slot {
        let cover_ptr = local_cover_url.as_deref();

        // 打印出来检查，确保没有 \\?\ 这种前缀
        if let Some(path) = cover_ptr {
            println!("Setting system cover path: {}", path);
        }

        controls.set_metadata(MediaMetadata {
            title: Some(&title),
            artist: Some(&artist),
            album: Some(&album),
            cover_url: local_cover_url.as_deref(),
            duration: None,
        }).map_err(|e| {
            let err_msg = format!("{:?}", e);
            // 如果还是报错，尝试不带封面发送，至少保证程序不崩
            eprintln!("Metadata failed: {}", err_msg);
            err_msg
        })?;
    }

    Ok(())
}

#[tauri::command]
pub async fn update_system_status(
    control_state: State<'_, PlayerControls>,
    is_paused: bool,
    progress: f64,
) -> Result<(), String> {
    let mut control_slot = control_state.0.lock().unwrap();
    let progress_pos = Some(MediaPosition(Duration::from_secs_f64(progress)));

    if let Some(ref mut controls) = *control_slot {
        let status = if is_paused {
            MediaPlayback::Paused { progress: progress_pos }
        } else {
            MediaPlayback::Playing { progress: progress_pos }
        };
        controls.set_playback(status).map_err(|e| format!("{:?}", e))?;
    }

    Ok(())
}
