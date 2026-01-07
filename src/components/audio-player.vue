<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { isPaused } from "../scripts/globals";
import { startProgressCollection, stopProgressCollection } from "../scripts/progress-collector";

/// This function asks Rust to play the audio
export async function handleSongPlayback(path: String) {
    try {
        await invoke('play_song', { path: path });
        isPaused.value = false;
        startProgressCollection();
        console.log("Playing the song...");
    } catch (error) {
        console.log("Something went wrong: ", error);
    }
}

/// This function asks Rust to play/pause the audio
export async function handleToggle() {
    try {
        isPaused.value = await invoke('toggle_playback', { });
        stopProgressCollection();
    } catch (error) {
        console.log("Something went wrong: ", error);
    }
}

</script>