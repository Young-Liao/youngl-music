<template>
    <div class="nyx-player-bar" :class="{ 'is-dragging': isDragging }">
        <div
            class="scrub-container"
            ref="progressBar"
            @mousedown="startScrub"
            @mouseenter="showHover = true"
            @mouseleave="showHover = false"
            @mousemove="updateHoverTime"
        >
            <div class="track-base">
                <!-- 1. The Locator: Vertical line following the mouse -->
                <div
                    v-show="showHover && !isDragging"
                    class="track-locator"
                    :style="{ left: hoverPercent + '%' }"
                ></div>

                <!-- 2. Hover Preview Fill -->
                <div v-show="showHover" class="track-hover" :style="{ width: hoverPercent + '%' }"></div>

                <!-- 3. Main Progress Fill -->
                <div class="track-fill" :style="{ width: progressPercent + '%' }"></div>

                <!-- 4. The Handle: Always visible with breathing animation -->
                <div class="track-handle" :style="{ left: progressPercent + '%' }"></div>
            </div>

            <!-- Floating Tooltip: Responsive text color and slide-up animation -->
            <div
                class="floating-time"
                :class="isDragging || showHover ? 'show-floating-time' : 'hide-floating-time'"
                :style="{ left: (isDragging ? progressPercent : hoverPercent) + '%' }"
            >
                {{ formatTime(isDragging ? currentTime : hoverTime) }}
            </div>
        </div>

        <div class="time-display">
            <span class="time-current">{{ formatTime(currentTime) }}</span>
            <span class="time-divider">/</span>
            <span class="time-total">{{ formatTime(currentMetadata.totalDuration) }}</span>
        </div>
    </div>
</template>

<script setup lang="ts">
import {ref, computed} from "vue";
// import { invoke } from "@tauri-apps/api/core";
import {currentMetadata, currentTime, lockCurrentTime} from "../scripts/globals";
import {setPosition, startProgressCollection, stopProgressCollection} from "../scripts/playback/progress-controller.ts";
import {checkAudioAvailability} from "../scripts/playback/audio-playback.ts";

const progressBar = ref<HTMLElement | null>(null);
const isDragging = ref(false);
const showHover = ref(false);
const hoverTime = ref(0);

// Calculate percentage for CSS
const progressPercent = computed(() => {
    if (currentMetadata.value.totalDuration <= 0) return 0;
    return (currentTime.value / currentMetadata.value.totalDuration) * 100;
});

const hoverPercent = computed(() => {
    if (currentMetadata.value.totalDuration <= 0) return 0;
    return (hoverTime.value / currentMetadata.value.totalDuration) * 100;
});

// --- Mouse Interaction Logic ---

const updateTimeFromEvent = (e: MouseEvent): number => {
    if (!progressBar.value) return 0;
    const rect = progressBar.value.getBoundingClientRect();
    const x = e.clientX - rect.left;
    // Clamp between 0 and 1
    const percentage = Math.max(0, Math.min(1, x / rect.width));
    return percentage * currentMetadata.value.totalDuration;
};

const updateHoverTime = (e: MouseEvent) => {
    if (isDragging.value) return; // Don't update hover if we are dragging
    hoverTime.value = updateTimeFromEvent(e);
};

const startScrub = async (e: MouseEvent) => {
    isDragging.value = true;
    // Pause the timer update loop while dragging so it doesn't fight the user
    // (You might want to implement a flag in your globals like `isScrubbing`)

    // Update UI immediately
    currentTime.value = updateTimeFromEvent(e);

    const onMouseMove = (moveEvent: MouseEvent) => {
        currentTime.value = updateTimeFromEvent(moveEvent);
    };

    stopProgressCollection()

    const onMouseUp = async () => {
        window.removeEventListener('mousemove', onMouseMove);
        window.removeEventListener('mouseup', onMouseUp);

        let now_value = currentTime.value;
        try {
            lockCurrentTime.value = true;
            if (await checkAudioAvailability('history')) {
                if (now_value > currentMetadata.value.totalDuration) {
                    console.log("Position gets out of bound.")
                } else {
                    console.log("Setting position to ", now_value);
                    // Call the rust backend to change the position.
                    await setPosition(now_value);
                    currentTime.value = now_value;
                    hoverTime.value = now_value;
                }
                startProgressCollection()
            }
            lockCurrentTime.value = false;
        } catch (e) {
            console.log("E: ", e);
        }

        isDragging.value = false;
    };

    window.addEventListener('mousemove', onMouseMove);
    window.addEventListener('mouseup', onMouseUp);
};

// Helper for 00:00 format
const formatTime = (time: number) => {
    const seconds = Math.floor(time % 60);
    const minutes = Math.floor(time / 60 % 60);
    const hours = Math.floor(time / 3600);

    const displaySeconds = seconds.toString().padStart(2, '0');
    const displayMinutes = minutes.toString().padStart(2, '0');
    const displayHours = hours.toString().padStart(2, '0');

    const MM_SS = `${displayMinutes}:${displaySeconds}`;
    if (time >= 3600) {
        return `${displayHours}:${MM_SS}`;
    } else {
        return MM_SS;
    }
}

</script>

<style scoped src="../styles/components/progressbar-style.css"></style>
