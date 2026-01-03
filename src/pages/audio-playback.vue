<script setup lang="ts">
import { ref } from "vue";
import AudioPicker from "../components/audio-picker.vue";
import AudioProgressbar from "../components/audio-progressbar.vue";
import { handleSongPlayback, handleToggle, isPaused } from "../components/audio-player.vue";


const selected = ref<string | null>(null);

/// This function let the audio-player play the specified audio file.
const handleSelection = (path: string) => {
    console.log("Chose: ", path);
    selected.value = path;
    handleSongPlayback(selected.value);
}

</script>

<template>
<AudioPicker @file-selected="handleSelection" />
<button @click="handleToggle"><i :class="isPaused ? 'bi bi-play-fill' : 'bi bi-pause-fill'"></i>{{ isPaused ? 'Play' : 'Pause' }}</button>
<label>Current Audio File: {{ selected }}</label>
<AudioProgressbar />

</template>