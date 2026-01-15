<script setup lang="ts">
import { onMounted, ref } from "vue";
import AudioPicker from "../components/audio-picker.vue";
import AudioProgressbar from "../components/audio-progressbar.vue";
import { handleSongPlayback, handleToggle, checkAudio } from "../components/audio-player.vue";
import {isPaused, currentTime, noAudio} from "../scripts/globals"
import { listen } from "@tauri-apps/api/event"
import { stopProgressCollection } from "../scripts/progress/progress-collector.ts";
import {invoke} from "@tauri-apps/api/core";
import {toggleAudioPlayback} from "../scripts/playback/audio-playback.ts";

const selected = ref<string | null>(null);

/// This function let the audio-player play the specified audio file.
onMounted(async () => {
    await listen('file-selected', async (event) => {
        const path = event.payload;
        try {
            await invoke('load_song', { path });
            console.log("The audio has been loaded.")
            await toggleAudioPlayback();
            console.log("The audio has been playing.")

        } catch (e) {
            console.log("Failed to load audio: ", e);
        }
    })
})

</script>

<template>
<AudioPicker />
<button @click="handleToggle"><i :class="isPaused ? 'bi bi-play-fill' : 'bi bi-pause-fill'"></i>{{ isPaused ? 'Play' : 'Pause' }}</button>
<label>Current Audio File: {{ selected }}</label>
<AudioProgressbar />

</template>