<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { isPaused } from '../scripts/globals';

const appWindow = getCurrentWindow();

const minimize = () => appWindow.minimize();
const toggleMax = () => appWindow.toggleMaximize();
const close = () => appWindow.close();

/**
 * Initiates native window dragging.
 * We trigger this on mousedown.
 */
const startWindowDrag = async () => {
    await appWindow.startDragging();
};
</script>

<template>
    <!-- Use @mousedown to trigger the native drag -->
    <div class="titlebar" @mousedown="startWindowDrag">
        <div class="traffic-lights">
            <!-- Use .stop to prevent the drag from starting when clicking buttons -->
            <button class="win-btn close" @click="close">
                <i class="bi bi-x-lg"></i>
            </button>
            <button class="win-btn min" @click="minimize">
                <i class="bi bi-dash-lg"></i>
            </button>
            <button class="win-btn max" @click="toggleMax">
                <i class="bi bi-arrows-angle-expand"></i>
            </button>
        </div>

        <!-- app-title has pointer-events: none so mousedown passes through to the bar -->
        <div class="app-title" :class="{ 'playing': !isPaused }">
            YoungL Music
        </div>

        <div class="spacer"></div>
    </div>
</template>

<style scoped>
.titlebar {
    height: 44px;
    width: 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 16px;
    z-index: 9999;
    user-select: none;
    cursor: default;
    position: relative;
    /* Optional: add a subtle bottom border to define the bar */
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.app-title {
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
    font-size: 11px;
    font-weight: 800;
    letter-spacing: 1px;
    color: rgba(255, 255, 255, 0.3);
    transition: color 0.5s ease, text-shadow 0.5s ease;
    pointer-events: none; /* Mouse events pass through to the titlebar */
    text-transform: uppercase;
}

.app-title.playing {
    color: var(--accent-primary);
    text-shadow: 0 0 10px var(--accent-primary);
}

.traffic-lights {
    display: flex;
    gap: 8px;
    position: relative;
    z-index: 10000;
}

.win-btn {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    border: none;
    padding: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    color: transparent;
    transition: 0.2s;
    background: rgba(255, 255, 255, 0.15);
    cursor: pointer;
}

.win-btn:hover { color: rgba(0, 0, 0, 0.6); }
.win-btn.close:hover { background: #ff5f56; }
.win-btn.min:hover { background: #ffbd2e; }
.win-btn.max:hover { background: #27c93f; }

.spacer { width: 50px; }
</style>
