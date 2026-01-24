use std::path::Path;
use std::collections::HashMap;
use std::io::BufRead;
use std::sync::Mutex;
use audiotags::Tag;
use base64::{engine::general_purpose, Engine as _};
use id3::{Tag as Id3Tag, TagLike};
use metaflac::Tag as FlacTag;
use lazy_static::lazy_static;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LyricLine {
    pub time: f64,
    pub text: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub cover: Option<String>,
    pub lyrics: Vec<LyricLine>,
    pub total_duration: f64,
}

lazy_static! {
    static ref METADATA_CACHE: Mutex<HashMap<String, AudioMetadata>> = Mutex::new(HashMap::new());
}

/// TIER 1: Parse raw LRC string into structured data
pub fn load_lyrics_from_str(content: String) -> Vec<LyricLine> {
    let mut lyrics = Vec::new();

    // Check if it's plain text (unsynced) or LRC (synced)
    if !content.contains('[') {
        return vec![LyricLine {
            time: 0.0,
            text: content.trim().to_string(),
        }];
    }

    for line in content.lines() {
        if let (Some(start), Some(end)) = (line.find('['), line.find(']')) {
            let time_str = &line[start + 1..end];
            let text = &line[end + 1..].trim();

            let parts: Vec<&str> = time_str.split(':').collect();
            if parts.len() >= 2 {
                let mins: f64 = parts[0].parse().unwrap_or(-1.0);
                let secs: f64 = parts[1].parse().unwrap_or(-1.0);

                if mins >= 0.0 && secs >= 0.0 {
                    lyrics.push(LyricLine {
                        time: mins * 60.0 + secs,
                        text: text.to_string(),
                    });
                }
            }
        }
    }
    lyrics.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap_or(std::cmp::Ordering::Equal));
    lyrics
}

/// TIER 2: Load from external .lrc sidecar file
#[tauri::command]
pub fn load_lyrics_from_lrc(path: &str) -> Option<Vec<LyricLine>> {
    let lrc_path = Path::new(path).with_extension("lrc");
    std::fs::read_to_string(lrc_path).ok().map(load_lyrics_from_str)
}

/// TIER 3: Load from embedded file tags (MP3/FLAC/M4A/OGG)
#[tauri::command]
pub fn load_lyrics_from_song(path: &str) -> Option<Vec<LyricLine>> {
    let ext = Path::new(path).extension()?.to_str()?.to_lowercase();

    let tag: Id3Tag;
    match ext.as_str() {
        "mp3" => {
            tag = Id3Tag::read_from_path(path).ok()?;
            tag.lyrics().next().map(|l| load_lyrics_from_str(l.text.clone()))
        },
        "flac" => {
            let tag = FlacTag::read_from_path(path).ok()?;
            tag.vorbis_comments()?
                .get("LYRICS")
                .and_then(|v| v.first())
                .map(|s| load_lyrics_from_str(s.clone()))
        },
        "m4a" | "mp4" => {
            let tag = mp4ameta::Tag::read_from_path(path).ok()?;
            tag.lyrics().map(|s| load_lyrics_from_str(s.to_string()))
        },
        "ogg" => {
            // Ogg Vorbis doesn't have a dedicated high-level crate as simple as the others,
            // so we attempt a raw read via a secondary check or return None for sidecar fallback.
            None
        },
        _ => None,
    }
}
