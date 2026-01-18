<script setup lang="ts">
import {ref, computed} from "vue";
import {isPaused, playbackHistory, playbackMode, playlist, volume} from "../scripts/globals";
import {loadAudio, toggleAudioPlayback} from "../scripts/playback/audio-playback.ts";
import {getNextValidAudio, getPrevValidAudio} from "../scripts/files/playlist.ts";

const lastVolume = ref(volume.value || 80); // Store volume for unmuting

const toggleMode = () => playbackMode.value = (playbackMode.value + 1) % 3;

const toggleMute = () => {
    if (volume.value > 0) {
        lastVolume.value = volume.value;
        volume.value = 0;
    } else {
        volume.value = lastVolume.value || 80;
    }
};

const modeIcon = computed(() => {
    switch (playbackMode.value) {
        case 0:
            return 'bi-shuffle';
        case 1:
            return 'bi-repeat';
        default:
            return 'bi-repeat-1';
    }
});

const volumeIcon = computed(() => {
    if (volume.value === 0) return 'bi-volume-mute-fill';
    if (volume.value < 50) return 'bi-volume-down-fill';
    return 'bi-volume-up-fill';
});

const skipStart = async () => {
    console.log(playlist.value);
    let file = await getPrevValidAudio();
    await loadAudio(file);
}

const skipEnd = async () => {
    console.log(playlist.value);
    let file = await getNextValidAudio(playbackMode.value, true);
    await loadAudio(file);
}

</script>

<template>
    <div class="nyx-controls-deck">
        <!-- LEFT: Utility -->
        <div class="sector left">
            <button class="mini-btn active" @click.stop="toggleMode">
                <i :class="`bi ${modeIcon}`"></i>
            </button>
            <button class="mini-btn" title="Lyrics">
                <i class="bi bi-chat-right-quote"></i>
            </button>
        </div>

        <!-- CENTER: Core Transport (Tighter gaps) -->
        <div class="sector center">
            <button class="nav-btn" @click.stop="skipStart"
                    :class="playbackHistory.length > 1 ? 'nav-btn-hoverable' : ''">
                <i class="bi bi-skip-start-fill"></i>
            </button>

            <button class="hero-play-btn" @click.stop="toggleAudioPlayback" :class="{ 'playing': !isPaused }">
                <i :class="isPaused ? 'bi bi-play-fill' : 'bi bi-pause-fill'"></i>
            </button>

            <button class="nav-btn" @click.stop="skipEnd"
                    :class="playlist.length > 1 ? 'nav-btn-hoverable' : ''">
                <i class="bi bi-skip-end-fill"></i>
            </button>
        </div>

        <!-- RIGHT: Environment (Horizontal + Click to Mute) -->
        <div class="sector right">
            <div class="horizontal-volume" :style="{ '--vol-percent': volume + '%' }">
                <button class="mute-toggle" @click.stop="toggleMute">
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
