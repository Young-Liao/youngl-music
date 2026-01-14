<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import {isPaused, noAudio, playbackHistory} from "../scripts/globals";
import { startProgressCollection, stopProgressCollection } from "../scripts/progress-collector";
import {emit, listen} from "@tauri-apps/api/event";

/// This function asks Rust to play the audio
export const handleSongPlayback = async (path: string) => {
    try {
        await invoke('play_song', { path: path });
        isPaused.value = false;
        startProgressCollection();
        console.log("Playing the song...");
        noAudio.value = false;
        playbackHistory.value.push(path);
    } catch (error) {
        console.log("Something went wrong: ", error);
    }
}

export const checkAudio = async (): Promise<boolean> => {
    if (!noAudio.value) return true;

    console.log("No audio... requesting file selection");

    // 1. Create a promise that resolves when the file selection is finished
    const selectionTask = new Promise<boolean>((resolve) => {
        // Listen for a one-time event from whoever handles the file picking
        const unlisten = listen('file-selection-result', (event) => {
            console.log("Result got.");
            unlisten.then(fn => fn()); // Clean up listener
            resolve(event.payload as boolean);
        });

        // 2. Trigger the selection UI
        emit('need-file-selection');

        // 3. Timeout safety (optional): cancel if user takes > 2 minutes
        setTimeout(() => resolve(false), 120000);
    });

    return await selectionTask;
}

/// This function asks Rust to play/pause the audio
export const handleToggle = async () => {
    if (await checkAudio()) {
        try {
            isPaused.value = await invoke('toggle_playback', {});
            if (isPaused.value) stopProgressCollection();
            else startProgressCollection();
        } catch (error) {
            console.log("Something went wrong: ", error);
        }
    }
}

</script>