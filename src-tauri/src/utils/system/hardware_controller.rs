use base64::engine::general_purpose;
use base64::Engine;
use std::fs;
use std::sync::Mutex;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager, State, Url};

// --- 条件导入：仅在非移动端使用 souvlaki ---
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use souvlaki::{
    MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, MediaPosition, PlatformConfig,
};

use crate::utils::song::AudioState;

/// Set the volume (这个是通用的，不用改)
#[tauri::command]
pub fn set_volume(volume: f32, state: State<'_, AudioState>) -> Result<(), String> {
    let sink = state.sink.lock().unwrap();
    sink.set_volume(volume / 100.);
    Ok(())
}

// --- 状态结构体：在移动端内部存储 None ---
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub struct PlayerControls(pub Mutex<Option<MediaControls>>);

#[cfg(any(target_os = "android", target_os = "ios"))]
pub struct PlayerControls(pub Mutex<Option<String>>); // 移动端只是个占位符

#[tauri::command]
pub async fn update_system_metadata(
    app: AppHandle,
    state: State<'_, PlayerControls>,
    title: String,
    artist: String,
    album: String,
    cover: Option<String>,
) -> Result<(), String> {
    // 如果是 Android/iOS，直接返回 Ok，不做任何 souvlaki 操作
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        // 移动端建议使用 tauri-plugin-media 处理元数据
        // 这里暂时静默返回，防止编译报错
        return Ok(());
    }

    // --- 以下是桌面端逻辑 ---
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let mut control_slot = state.0.lock().unwrap();

        // 1. 初始化
        if control_slot.is_none() {
            let window = app
                .get_webview_window("main")
                .ok_or_else(|| "Main window not found".to_string())?;

            #[cfg(target_os = "windows")]
            let hwnd = Some(window.hwnd().map_err(|e| e.to_string())?.0 as *mut std::ffi::c_void);
            #[cfg(not(target_os = "windows"))]
            let hwnd = None;

            let config = PlatformConfig {
                dbus_name: "com.young.youngl-music",
                display_name: "YoungL Music",
                hwnd,
            };

            let mut c = MediaControls::new(config).map_err(|e| format!("{:?}", e))?;
            c.set_playback(MediaPlayback::Playing { progress: None })
                .ok();

            let handle = app.clone();
            c.attach(move |event| {
                let signal = match event {
                    MediaControlEvent::Toggle
                    | MediaControlEvent::Play
                    | MediaControlEvent::Pause => "Toggle",
                    MediaControlEvent::Next => "Next",
                    MediaControlEvent::Previous => "Previous",
                    _ => "Unknown",
                };
                let _ = handle.emit("system-media-event", signal);
            })
            .map_err(|e| format!("{:?}", e))?;

            *control_slot = Some(c);
        }

        // 2. 转换 Base64
        let mut local_cover_url = None;
        if let Some(base64_str) = cover {
            let base64_data = base64_str.split(',').last().unwrap_or(&base64_str);
            if let Ok(bytes) = general_purpose::STANDARD.decode(base64_data) {
                let temp_path = std::env::temp_dir().join("yt_music_cover.jpg");
                let _ = fs::remove_file(&temp_path);
                if fs::write(&temp_path, bytes).is_ok() {
                    local_cover_url = Some(temp_path.to_str().unwrap().to_string());
                }
            }
        }

        // 3. 提交元数据
        if let Some(ref mut controls) = *control_slot {
            controls
                .set_metadata(MediaMetadata {
                    title: Some(&title),
                    artist: Some(&artist),
                    album: Some(&album),
                    cover_url: local_cover_url.as_deref(),
                    duration: None,
                })
                .map_err(|e| format!("{:?}", e))?;
        }
        Ok(())
    }
}

#[tauri::command]
pub async fn update_system_status(
    control_state: State<'_, PlayerControls>,
    is_paused: bool,
    progress: f64,
) -> Result<(), String> {
    // Android 直接跳过
    #[cfg(any(target_os = "android", target_os = "ios"))]
    return Ok(());

    // 桌面端逻辑
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    {
        let mut control_slot = control_state.0.lock().unwrap();
        let progress_pos = Some(MediaPosition(Duration::from_secs_f64(progress)));

        if let Some(ref mut controls) = *control_slot {
            let status = if is_paused {
                MediaPlayback::Paused {
                    progress: progress_pos,
                }
            } else {
                MediaPlayback::Playing {
                    progress: progress_pos,
                }
            };
            controls
                .set_playback(status)
                .map_err(|e| format!("{:?}", e))?;
        }
        Ok(())
    }
}
