<script setup lang="ts">
import { isPaused } from "../scripts/globals";
import { toggleAudioPlayback } from "../scripts/playback/audio-playback.ts";
import { handleSelectionNeeded } from "../scripts/files/file-selection.ts";

// Placeholder functions for buttons that don't have logic yet
const nextTrack = () => console.log("Next track clicked");
const prevTrack = () => console.log("Prev track clicked");
</script>

<template>
    <div class="controls-wrapper">
        <div class="main-controls">
            <button class="icon-btn secondary" title="Shuffle">
                <i class="bi bi-shuffle"></i>
            </button>

            <button class="icon-btn" @click="prevTrack">
                <i class="bi bi-skip-start-fill"></i>
            </button>

            <button class="play-btn" @click="toggleAudioPlayback">
                <i :class="isPaused ? 'bi bi-play-fill' : 'bi bi-pause-fill'"></i>
            </button>

            <button class="icon-btn" @click="nextTrack">
                <i class="bi bi-skip-end-fill"></i>
            </button>

            <button class="icon-btn secondary" title="Loop">
                <i class="bi bi-repeat"></i>
            </button>
        </div>

        <div class="util-controls">
            <button class="text-btn" @click="handleSelectionNeeded">
                <i class="bi bi-folder2-open"></i> Open File
            </button>
        </div>
    </div>
</template>

<style scoped src="../styles/components/audio-controls.css"></style>
