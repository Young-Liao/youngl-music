<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
    path: string | null
}>();

// Extract a cleaner title from the file path
const trackTitle = computed(() => {
    if (!props.path) return "No Audio Selected";
    // Get filename, remove extension
    const filename = props.path.split(/[\\/]/).pop() || "";
    return filename.replace(/\.[^/.]+$/, "");
});
</script>

<template>
    <div class="track-info-container">
        <div class="album-art-placeholder">
            <i class="bi bi-music-note-beamed"></i>
        </div>

        <div class="text-details">
            <h2 class="track-title" :title="trackTitle">{{ trackTitle }}</h2>
            <p class="track-artist">Unknown Artist</p>
        </div>
    </div>
</template>

<style scoped src="../styles/components/track-info.css"></style>
