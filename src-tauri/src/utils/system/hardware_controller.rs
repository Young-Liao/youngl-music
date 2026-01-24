use std::fs;
use std::sync::Mutex;
use std::time::Duration;
use base64::Engine;
use base64::engine::general_purpose;
use tauri::{AppHandle, Emitter, Manager, State};
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
            dbus_name: "youngl_music_v3", // 每次改代码建议微调名字，强制 Windows 刷新
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

    // 2. 转换 Base64 为本地文件路径 (关键逻辑)
    let mut local_cover_url = None;
    if let Some(base64_str) = cover {
        // 过滤前缀 data:image/...;base64,
        let base64_data = base64_str.split(',').last().unwrap_or(&base64_str);

        if let Ok(bytes) = general_purpose::STANDARD.decode(base64_data) {
            if let Ok(cache_dir) = app.path().app_cache_dir() {
                let cover_path = cache_dir.join("current_cover.jpg");
                let _ = fs::remove_file(&cover_path); // 确保旧图被替换
                if fs::write(&cover_path, bytes).is_ok() {
                    // Windows 必须使用 file:/// 协议，斜杠需统一为正斜杠
                    let path_str = cover_path.to_string_lossy().replace("\\", "/");
                    local_cover_url = Some(format!("file:///{}", path_str));
                }
            }
        }
    }

    println!("Len: {}", local_cover_url.as_deref().unwrap().len());
    // 3. 提交元数据 (这里是之前报错的源头！)
    if let Some(ref mut controls) = *control_slot {
        controls.set_metadata(MediaMetadata {
            title: Some(&title),
            artist: Some(&artist),
            album: Some(&album),
            // ✅ 注意！这里必须用 local_cover_url，而不是原始的 cover 变量
            cover_url: local_cover_url.as_deref(),
            duration: None,
        }).map_err(|e| format!("Windows API Error: {:?}", e))?;
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
