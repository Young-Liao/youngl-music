<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import {isPaused, noAudio} from "../scripts/globals";
import { startProgressCollection, stopProgressCollection } from "../scripts/progress-collector";
import {emit} from "@tauri-apps/api/event";

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
    if (noAudio.value) {
        console.log("No audio...");
        await emit('need-file-selection');
    }
    // Canceled selection
    if (noAudio.value) {
        return;
    }
    try {
        isPaused.value = await invoke('toggle_playback', { });
        stopProgressCollection();
    } catch (error) {
        console.log("Something went wrong: ", error);
    }
}

</script>