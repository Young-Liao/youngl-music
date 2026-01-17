<script setup lang="ts">
import { ref, computed } from "vue";
import { isPaused, volume } from "../scripts/globals";
import { toggleAudioPlayback } from "../scripts/playback/audio-playback.ts";

const playbackMode = ref(0);
const lastVolume = ref(volume.value || 80); // Store volume for unmuting

const toggleMode = () => playbackMode.value = (playbackMode.value + 1) % 4;

const toggleMute = () => {
    if (volume.value > 0) {
        lastVolume.value = volume.value;
        volume.value = 0;
    } else {
        volume.value = lastVolume.value || 80;
    }
};

const modeIcon = computed(() => {
    switch(playbackMode.value) {
        case 1: return 'bi-shuffle';
        case 2: return 'bi-repeat-1';
        case 3: return 'bi-repeat';
        default: return 'bi-distribute-horizontal';
    }
});

const volumeIcon = computed(() => {
    if (volume.value === 0) return 'bi-volume-mute-fill';
    if (volume.value < 50) return 'bi-volume-down-fill';
    return 'bi-volume-up-fill';
});
</script>

<template>
    <div class="nyx-controls-deck">
        <!-- LEFT: Utility -->
        <div class="sector left">
            <button class="mini-btn" @click="toggleMode" :class="{ 'active': playbackMode !== 0 }">
                <i :class="`bi ${modeIcon}`"></i>
            </button>
            <button class="mini-btn" title="Lyrics">
                <i class="bi bi-chat-right-quote"></i>
            </button>
        </div>

        <!-- CENTER: Core Transport (Tighter gaps) -->
        <div class="sector center">
            <button class="nav-btn"><i class="bi bi-skip-start-fill"></i></button>

            <button class="hero-play-btn" @click="toggleAudioPlayback" :class="{ 'playing': !isPaused }">
                <i :class="isPaused ? 'bi bi-play-fill' : 'bi bi-pause-fill'"></i>
            </button>

            <button class="nav-btn"><i class="bi bi-skip-end-fill"></i></button>
        </div>

        <!-- RIGHT: Environment (Horizontal + Click to Mute) -->
        <div class="sector right">
            <div class="horizontal-volume" :style="{ '--vol-percent': volume + '%' }">
                <button class="mute-toggle" @click="toggleMute">
                    <i :class="`bi ${volumeIcon}`"></i>
                </button>
                <div class="slider-capsule">
                    <div class="track-base">
                        <div class="track-fill"></div>
                    </div>
                    <input
                        type="range"
                        min="0" max="100"
                        v-model.number="volume"
                        class="volume-input"
                    />
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped src="../styles/components/audio-controls.css"></style>
