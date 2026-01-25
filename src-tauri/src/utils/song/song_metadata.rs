use audiotags::{AudioTagEdit, Tag};
use base64::engine::general_purpose;
use base64::Engine;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(serde::Serialize, Clone)]
pub struct AudioMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub cover: Option<String>, // Base64 encoded image
    pub total_duration: f64,
}

lazy_static! {
    // Cache maps file path -> AudioMetadata
    static ref METADATA_CACHE: Mutex<HashMap<String, AudioMetadata>> = Mutex::new(HashMap::new());
}

pub fn get_metadata(path: &String, total_duration: f64) -> AudioMetadata {
    // 1. Check if it's already in cache
    {
        let cache = METADATA_CACHE.lock().unwrap();
        if let Some(data) = cache.get(path) {
            let mut ret = data.clone();
            ret.total_duration = total_duration;
            return ret; // Requires AudioMetadata to derive Clone
        }
    }

    // 2. If not found, do the heavy lifting
    let metadata = match Tag::new().read_from_path(&path) {
        Ok(tag) => {
            let cover_base64 = tag.album_cover().map(|cover| {
                let b64 = general_purpose::STANDARD.encode(cover.data);
                let mime = match cover.mime_type {
                    audiotags::MimeType::Png => "image/png",
                    _ => "image/jpeg",
                };
                format!("data:{};base64,{}", mime, b64)
            });
            AudioMetadata {
                title: tag.title().map(|s| s.to_string()),
                artist: tag.artist().map(|s| s.to_string()),
                album: tag.album().map(|a| a.title.to_string()),
                cover: cover_base64,
                total_duration,
            }
        }
        Err(_) => AudioMetadata {
            title: None,
            artist: None,
            album: None,
            cover: None,
            total_duration,
        },
    };

    // 3. Save to cache for next time
    let mut cache = METADATA_CACHE.lock().unwrap();
    cache.insert(path.clone(), metadata.clone());

    metadata
}

/// Get the metadata of some files.
#[tauri::command]
pub fn fetch_metadata(paths: Vec<String>) -> Result<Vec<AudioMetadata>, String> {
    Ok(paths.iter().map(|path| get_metadata(path, 0.)).collect())
}
