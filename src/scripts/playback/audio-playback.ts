import {invoke} from "@tauri-apps/api/core";

export const toggleAudioPlayback = async () => {
    await invoke('toggle_playback');
}