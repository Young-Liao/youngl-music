<template>
<div class="progress_container">
    <input
        type="range",
        min="0"
        :max="totalDuration"
        :value="currentTime"
        class="from-range"  
    />
    <span>{{ formatTime(currentTime) }} / {{ formatTime(totalDuration) }}</span>
</div>
</template>
<script setup lang="ts">
import { ref, onMounted } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { listen } from "@tauri-apps/api/event"

const currentTime = ref(0);
const totalDuration = ref(0);

async function updateProgress() {
    const stats = await invoke<{position: number, duration: number}>('get_playback_progress'); 
    currentTime.value = stats.position;
    totalDuration.value = stats.duration;
}

const formatTime = (time: number) => {
    const seconds = Math.floor(time % 60);
    const minutes = Math.floor(time / 60 % 60);
    const hours = Math.floor(time / 3600);

    const displaySeconds = seconds.toString().padStart(2, '0');
    const displayMinutes = minutes.toString().padStart(2, '0');
    const displayHours = hours.toString().padStart(2, '0');

    const MM_SS = `${displayMinutes}:${displaySeconds}`;
    if (time >= 3600) {
        return `${displayHours}:${MM_SS}`;
    } else {
        return MM_SS;
    }
}

onMounted(async () => {
    /// Start the Timer
    setInterval(updateProgress, 1000);
    const unlisten = await listen('playback-finished', () => {
        currentTime.value = 0;
    })
})

</script>