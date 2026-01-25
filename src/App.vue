<!-- src/App.vue -->
<script setup lang="ts">
import {onUnmounted} from "vue";
import AudioPlayback from "./pages/audio-playback.vue";
import PlaylistPanel from "./components/playlist-panel.vue";
import {currentTheme, isPlaylistOpen, playlistCloseTimer} from "./scripts/globals.ts";
import {onMounted} from 'vue';
import {cancelPendingClose, initSystemListeners} from './scripts/utils/system-api';
import {destroyKeyboardEventHandler, initKeyboardEventHandler} from "./scripts/utils/keyboard-events.ts";
import {listen} from "@tauri-apps/api/event";

onMounted(() => {
    initSystemListeners();
    initKeyboardEventHandler()
});

onMounted(async () => {
    await listen("open-menu", () => {
        cancelPendingClose();
        isPlaylistOpen.value = true;
    })
    await listen("close-menu", () => {
        cancelPendingClose();
        playlistCloseTimer.value = setTimeout(() => {
            isPlaylistOpen.value = false;
        }, 3000);
    })
})

onUnmounted(() => {
    destroyKeyboardEventHandler();
})

</script>

<template>
    <div :class="currentTheme">
        <transition name="fade">
            <div
                v-if="isPlaylistOpen"
                class="sidebar-overlay"
                @click="isPlaylistOpen = false"
            ></div>
        </transition>
        <!-- The Global Playlist Button -->
        <button class="global-playlist-trigger" @click.stop="{ cancelPendingClose(); isPlaylistOpen = true; }" title="Open Queue">
            <i class="bi bi-music-note-list"></i>
        </button>

        <PlaylistPanel :isOpen="isPlaylistOpen" @close="isPlaylistOpen = false"/>

        <AudioPlayback/>
    </div>
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
