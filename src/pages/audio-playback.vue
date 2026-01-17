<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { resetStates } from "../scripts/playback/audio-playback.ts";

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

    await listen('playback-finished', async () => await resetStates());
});

onUnmounted(() => window.removeEventListener('resize', checkWindowSize));

const toggleMobileView = () => {
    if (!isWide.value) showLyricsMobile.value = !showLyricsMobile.value;
};
</script>

<template>
    <div class="window-shell" :class="currentTheme">
        <!-- The background gradient layer -->
        <div class="background-fx"></div>

        <div class="glass-layer">
            <!-- Window Movement handled in TitleBar -->
            <TitleBar />

            <div class="content-area" :class="{ 'layout-wide': isWide }">
                <!-- Middle Section: Visuals and Lyrics -->
                <div class="player-body">
                    <div class="left-pane">
                        <div class="visual-container" @click="toggleMobileView"
                             :class="{'hidden': !isWide && showLyricsMobile, 'clickable': !isWide}">
                            <TrackInfo :path="selected" />
                        </div>

                        <div class="lyrics-mobile-container" @click="toggleMobileView"
                             :class="{'active': !isWide && showLyricsMobile}">
                            <LyricsView />
                        </div>
                    </div>

                    <div class="right-pane" :class="{'visible': isWide}">
                        <LyricsView />
                    </div>
                </div>
            </div>

            <!-- Bottom Section: Full Width Controls -->
            <div class="controls-outer-wrapper">
                <AudioProgressbar />
                <AudioControls />
            </div>
        </div>
    </div>
</template>

<style scoped>
.window-shell {
    width: 100vw;
    height: 100vh;
    background: #050505; /* Solid base */
    overflow: hidden;
    position: relative;
}

.background-fx {
    position: absolute;
    inset: 0;
    background: var(--bg-gradient);
    filter: blur(100px);
    opacity: 0.8; /* Higher opacity for vivid colors */
    transition: background 1s ease;
    z-index: 0;
}

.glass-layer {
    position: relative;
    z-index: 1;
    width: 100%;
    height: 100%;
    /* Acrylic density: 0.8 opacity makes it look premium */
    background: rgba(10, 10, 10, 0.8);
    backdrop-filter: blur(50px) saturate(180%);
    display: flex;
    flex-direction: column;
}

.content-area {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 0 20px;
    overflow: hidden;
    transition: all 0.5s cubic-bezier(0.25, 0.8, 0.25, 1);
}

.player-body {
    flex: 1;
    display: flex;
    flex-direction: row;
    width: 100%;
    height: 100%;
}

.left-pane {
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
    transition: all 0.5s ease;
}

.right-pane {
    width: 0;
    opacity: 0;
    overflow: hidden;
    display: flex;
    /* Slide and fade animation */
    transition: width 0.5s ease, opacity 0.4s ease, transform 0.5s ease;
    transform: translateX(20px);
}

.right-pane.visible {
    width: 50%;
    opacity: 1;
    transform: translateX(0);
}

/* Full Width Bottom Controls */
.controls-outer-wrapper {
    width: 100%;
    padding: 20px 40px 40px 40px;
    box-sizing: border-box;
    z-index: 10;
    /* Gentle gradient at bottom to ground the controls */
    background: linear-gradient(to top, rgba(0,0,0,0.4) 0%, transparent 100%);
    transition: all 0.4s ease;
}

/* Mobile Toggles Animations */
.visual-container { transition: opacity 0.3s ease, transform 0.3s ease; }
.visual-container.hidden { opacity: 0; pointer-events: none; transform: scale(0.9); display: none; }
.visual-container.clickable { cursor: pointer; }

.lyrics-mobile-container { display: none; flex: 1; cursor: pointer; transition: opacity 0.3s ease; }
.lyrics-mobile-container.active { display: flex; animation: fadeIn 0.4s ease; }

@keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
}
</style>
