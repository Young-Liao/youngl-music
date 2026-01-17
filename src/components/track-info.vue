<script setup lang="ts">
import {currentMetadata} from "../scripts/globals.ts";

const props = defineProps<{
    path: string | null
}>();

</script>

<template>
    <div class="track-info-container">
        <img v-if="currentMetadata.cover"
             :src="currentMetadata.cover"
             class="album-art-placeholder"
             alt=""/>

        <div v-else class="album-art-placeholder">
            <i class="bi bi-music-note-beamed"></i>
        </div>

        <div class="text-details">
            <h2 class="track-title">{{ currentMetadata.title }}</h2>
            <p class="track-artist">{{ currentMetadata.artist }}</p>
        </div>
    </div>
</template>

<style scoped src="../styles/components/track-info.css"></style>
