<script setup lang="ts">
import { ref, computed } from "vue";
import { isPaused } from "../scripts/globals";
import { toggleAudioPlayback } from "../scripts/playback/audio-playback.ts";
import { handleSelectionNeeded } from "../scripts/files/file-selection.ts";

// --- State for New Buttons (Visual Logic Only) ---
// 0 = Order, 1 = Shuffle, 2 = Loop Single, 3 = Loop All
const playbackMode = ref(0);

const toggleMode = () => {
    playbackMode.value = (playbackMode.value + 1) % 4;
};

// Icons based on mode
const modeIcon = computed(() => {
    switch(playbackMode.value) {
        case 0: return 'bi-sort-down';
        case 1: return 'bi-shuffle';
        case 2: return 'bi-repeat-1';
        case 3: return 'bi-repeat';
        default: return 'bi-sort-down';
    }
});

const modeName = computed(() => {
    switch(playbackMode.value) {
        case 0: return 'Sequence';
        case 1: return 'Shuffle';
        case 2: return 'Loop One';
        case 3: return 'Loop All';
        default: return '';
    }
});

const toggleDesktopLyric = () => console.log("Toggle Desktop Lyric (Not Impl)");
const openPlaylist = () => console.log("Open Playlist (Not Impl)");
</script>

<template>
    <div class="controls-wrapper">
        <div class="main-controls">
            <button class="icon-btn secondary" @click="toggleMode" :title="modeName">
                <i :class="`bi ${modeIcon}`"></i>
                <div v-if="playbackMode !== 0" class="active-dot"></div>
            </button>

            <button class="icon-btn">
                <i class="bi bi-skip-start-fill"></i>
            </button>

            <button class="play-btn" @click="toggleAudioPlayback">
                <i :class="isPaused ? 'bi bi-play-fill' : 'bi bi-pause-fill'"></i>
            </button>

            <button class="icon-btn">
                <i class="bi bi-skip-end-fill"></i>
            </button>

            <button class="icon-btn secondary" @click="openPlaylist" title="Playlist">
                <i class="bi bi-music-note-list"></i>
            </button>
        </div>

        <div class="util-controls">
            <button class="text-btn" @click="toggleDesktopLyric">
                <i class="bi bi-chat-square-quote"></i> Lyrics
            </button>
            <button class="text-btn" @click="handleSelectionNeeded">
                <i class="bi bi-folder2-open"></i> Open
            </button>
        </div>
    </div>
</template>

<style scoped src="../styles/components/audio-controls.css"></style>
<style scoped>
/* Remove animations for window resizing as requested */
.content-area, .left-pane, .right-pane, .glass-layer {
    transition: none !important;
}

.window-shell {
    width: 100vw;
    height: 100vh;
    background: var(--bg-gradient); /* Theme gradient is the base */
    overflow: hidden;
    position: relative;
}

.background-fx {
    display: none; /* We now use the shell background for better gradient control */
}

.glass-layer {
    position: relative;
    z-index: 1;
    width: 100%;
    height: 100%;
    /* 0.8 opacity provides the "Acrylic" weight */
    background: rgba(10, 10, 10, 0.8);
    backdrop-filter: blur(50px) saturate(160%);
    display: flex;
    flex-direction: column;
}

.content-area {
    flex: 1;
    display: flex;
    padding: 20px;
    gap: 30px;
}

.controls-container {
    width: 100%;
    margin-top: auto;
    padding: 0;
}
</style>
