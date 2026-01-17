<!-- src/App.vue -->
<script setup lang="ts">
import { ref } from "vue";
import AudioPlayback from "./pages/audio-playback.vue";
import PlaylistPanel from "./components/playlist-panel.vue";

const isPlaylistOpen = ref(false);
</script>

<template>
    <!-- The Global Playlist Button -->
    <button class="global-playlist-trigger" @click="isPlaylistOpen = true" title="Open Queue">
        <i class="bi bi-music-note-list"></i>
    </button>

    <PlaylistPanel :isOpen="isPlaylistOpen" @close="isPlaylistOpen = false" />

    <AudioPlayback/>
</template>

<style>
.global-playlist-trigger {
    position: fixed;
    top: 55px; /* Just below the 44px title bar */
    right: 20px;
    z-index: 999;
    width: 40px;
    height: 40px;
    border-radius: 12px;
    background: rgba(255, 255, 255, 0.05);
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

.global-playlist-trigger:hover {
    background: var(--accent-primary);
    transform: scale(1.1) rotate(5deg);
    box-shadow: 0 0 20px var(--text-glow);
}
</style>
