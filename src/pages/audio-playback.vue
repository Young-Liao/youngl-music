<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { loadAudio, resetStates } from "../scripts/playback/audio-playback.ts";

import AudioProgressbar from "../components/audio-progressbar.vue";
import AudioControls from "../components/audio-controls.vue";
import TrackInfo from "../components/track-info.vue";
import TitleBar from "../components/title-bar.vue";
import LyricsView from "../components/lyrics-view.vue";

const selected = ref<string | null>(null);
const currentTheme = ref('theme-sunset');
const isWide = ref(false);
const showLyricsMobile = ref(false);

const themes = ['theme-sunset', 'theme-ocean', 'theme-midnight', 'theme-gold'];

// Updated logic: Wide mode is strictly for layout, not mobile toggle state
const checkWindowSize = () => {
    isWide.value = window.innerWidth > 750;
    if (isWide.value) showLyricsMobile.value = false;
};

const randomizeTheme = () => {
    currentTheme.value = themes[Math.floor(Math.random() * themes.length)];
};

onMounted(async () => {
    window.addEventListener('resize', checkWindowSize);
    checkWindowSize();

    // Random theme on startup
    randomizeTheme();

    await listen('file-selected', async (event) => {
        const path = event.payload as string;
        selected.value = path;
        randomizeTheme();
        await loadAudio(path);
    });

    await listen('playback-finished', async () => resetStates());
});

onUnmounted(() => window.removeEventListener('resize', checkWindowSize));

const toggleMobileView = () => {
    if (!isWide.value) showLyricsMobile.value = !showLyricsMobile.value;
};
</script>

<template>
    <div class="window-shell" :class="currentTheme">
        <div class="background-fx"></div>

        <div class="glass-layer">
            <TitleBar />

            <div class="content-area" :class="{ 'layout-wide': isWide }">

                <div class="left-pane">
                    <div class="visual-container" @click="toggleMobileView" :class="{'hidden': !isWide && showLyricsMobile, 'clickable': !isWide}">
                        <TrackInfo :path="selected" />
                    </div>

                    <div class="lyrics-mobile-container" @click="toggleMobileView" :class="{'active': !isWide && showLyricsMobile}">
                        <LyricsView />
                    </div>

                    <div class="controls-container">
                        <AudioProgressbar />
                        <AudioControls />
                    </div>
                </div>

                <div class="right-pane" :class="{'visible': isWide}">
                    <LyricsView />
                </div>

            </div>
        </div>
    </div>
</template>

<style scoped>
/* Update these specific classes in the <style> section */
.window-shell {
    width: 100vw;
    height: 100vh;
    background: transparent; /* Changed from #000 */
    overflow: hidden;
    position: relative;
}

.background-fx {
    position: absolute;
    inset: 0;
    background: var(--bg-gradient);
    filter: blur(100px); /* Increased blur */
    opacity: 0.6;
    transition: background 1s ease;
    z-index: 0;
}

.glass-layer {
    position: relative;
    z-index: 1;
    width: 100%;
    height: 100%;
    /* Darken slightly but keep it transparent enough for blur */
    background: rgba(10, 10, 10, 0.4);
    backdrop-filter: blur(40px) saturate(150%);
    display: flex;
    flex-direction: column;
}

/* --- FLUID LAYOUT (Flexbox) --- */
.content-area {
    flex: 1;
    display: flex;
    flex-direction: column; /* Default: Mobile Stack */
    padding: 0 20px 20px 20px;
    gap: 20px;
    transition: all 0.4s cubic-bezier(0.25, 0.8, 0.25, 1);
}

.left-pane {
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
    width: 100%;
    transition: all 0.4s ease;
}

.right-pane {
    width: 0;
    opacity: 0;
    overflow: hidden;
    transition: all 0.4s ease;
    /* Hide completely in mobile */
    display: none;
}

/* --- WIDE MODE (Desktop) --- */
.content-area.layout-wide {
    flex-direction: row; /* Switch to side-by-side */
    padding: 40px;
    gap: 40px;
}

.content-area.layout-wide .right-pane {
    display: flex;
    width: 50%; /* Take half space */
    opacity: 1;
}

.content-area.layout-wide .left-pane {
    width: 50%;
}

/* --- COMPONENT CONTAINERS --- */
.visual-container {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s ease;
}
.visual-container.hidden { display: none; }
.visual-container.clickable { cursor: pointer; }
.visual-container.clickable:active { transform: scale(0.96); }

/* Mobile Lyrics Overlay Logic */
.lyrics-mobile-container {
    display: none;
    flex: 1;
    cursor: pointer;
}
.lyrics-mobile-container.active { display: flex; }

.controls-container {
    width: 100%;
    /* Removed max-width to fill window as requested */
    margin-top: auto;
    padding-top: 20px;
}
</style>
