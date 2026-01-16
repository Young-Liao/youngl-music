<script setup lang="ts">
import { onMounted, ref } from "vue";
import AudioPicker from "../components/audio-picker.vue";
import AudioProgressbar from "../components/audio-progressbar.vue";
import {isPaused} from "../scripts/globals"
import { listen } from "@tauri-apps/api/event"
import {loadAudio, resetStates, toggleAudioPlayback} from "../scripts/playback/audio-playback.ts";

const selected = ref<string | null>(null);

/// This function let the song-player play the specified song file.
onMounted(async () => {
    await listen('file-selected', async (event) => {
        try {
            const path = event.payload;
            await loadAudio(path);

        } catch (e) {
            console.log("Failed to load song: ", e);
        }
    })

    await listen('playback-finished', async () => {
        resetStates();
    })
})

const handleToggle = async () => {
    await toggleAudioPlayback()
}

</script>

<template>
<AudioPicker />
<button @click="handleToggle"><i :class="isPaused ? 'bi bi-play-fill' : 'bi bi-pause-fill'"></i>{{ isPaused ? 'Play' : 'Pause' }}</button>
<label>Current Audio File: {{ selected }}</label>
<AudioProgressbar />

</template>