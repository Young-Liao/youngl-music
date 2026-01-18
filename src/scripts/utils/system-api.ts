import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { toggleAudioPlayback, skipEnd, skipStart } from "../playback/audio-playback";
import { currentMetadata, isPaused } from "../globals";

/**
 * Initializes listeners for System Media Keys (Fn + Media Keys)
 */
export async function initSystemMediaListeners() {
    // Listen for events emitted from Rust when the user interacts
    // with the Windows Media Overlay or physical keyboard keys
    await listen("system-media-toggle", () => {
        toggleAudioPlayback();
    });

    await listen("system-media-next", () => {
        skipEnd();
    });

    await listen("system-media-prev", () => {
        skipStart();
    });
}

/**
 * Updates the Windows "Now Playing" flyout
 */
export async function syncWithSystemMedia() {
    const meta = currentMetadata.value;

    // Send data to Rust to show in the Taskbar/Sound settings
    await invoke("update_system_metadata", {
        title: meta.title || "Unknown",
        artist: meta.artist || "Unknown Artist",
        album: meta.album || "Unknown Album",
        coverUrl: meta.cover || "" // This should be a data URL or local file path
    });
}

/**
 * Updates the Play/Pause button state in the Windows Overlay
 */
export async function updatePlaybackState() {
    // You would tell Rust if the status is "Playing" or "Paused"
    await invoke("update_system_playback_status", {
        status: isPaused.value ? "Paused" : "Playing"
    });
}