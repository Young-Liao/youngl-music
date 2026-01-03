<script lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export const isPaused = ref(true);

/// This function asks Rust to play the audio
export async function handleSongPlayback(path: String) {
    try {
        await invoke('play_song', { path: path });
        isPaused.value = false;
        console.log("Playing the song...");
    } catch (error) {
        console.log("Something went wrong: ", error);
    }
}

/// This function asks Rust to play/pause the audio
export async function handleToggle() {
    try {
        isPaused.value = await invoke('toggle_playback', { });
    } catch (error) {
        console.log("Something went wrong: ", error);
    }
}

</script>
<script setup lang="ts">
import { listen } from '@tauri-apps/api/event';
import { onMounted } from "vue";

onMounted(async () => {
    /// Listen for the 'playback-finished' event from Rust
    const unlisten = await listen('playback-finished', () => {
        isPaused.value = true;
    })
})

</script>