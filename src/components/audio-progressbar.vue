<template>
<div class="progress_container">
    <input
        type="range"
        min="0"
        :max="totalDuration"
        :value="currentTime"
        class="from-range"  
    />
    <span>{{ formatTime(currentTime) }} / {{ formatTime(totalDuration) }}</span>
</div>
</template>
<script setup lang="ts">
import { currentTime, totalDuration } from "../scripts/globals";

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

</script>