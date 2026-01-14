<script setup lang="ts">
import { onMounted, ref } from "vue";
import AudioPicker from "../components/audio-picker.vue";
import AudioProgressbar from "../components/audio-progressbar.vue";
import { handleSongPlayback, handleToggle, checkAudio } from "../components/audio-player.vue";
import {isPaused, currentTime, noAudio} from "../scripts/globals"
import { listen } from "@tauri-apps/api/event"
import { stopProgressCollection } from "../scripts/progress-collector";

const selected = ref<string | null>(null);

/// This function let the audio-player play the specified audio file.
const handleSelection = async (path: string) => {
    console.log("Chose: ", path);
    selected.value = path;
    await handleSongPlayback(selected.value);
}

const handleRestartPlayback = async (callback: (success: boolean) => void) => {
    try {
        const result = await checkAudio();
        callback(result);
    } catch (err) {
        callback(false);
    }
};

onMounted(async () => {
    /// Stop the timer when the playback is done.
    await listen('playback-finished', () => {
        stopProgressCollection();
        currentTime.value = 0;
        isPaused.value = true;
        noAudio.value = true;
    })
})

</script>

<template>
<AudioPicker @file-selected="handleSelection" />
<button @click="handleToggle"><i :class="isPaused ? 'bi bi-play-fill' : 'bi bi-pause-fill'"></i>{{ isPaused ? 'Play' : 'Pause' }}</button>
<label>Current Audio File: {{ selected }}</label>
<AudioProgressbar @restart-playback="handleRestartPlayback" />

</template>