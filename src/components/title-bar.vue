<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { isPaused } from '../scripts/globals';

const appWindow = getCurrentWindow();

// Window Actions
const minimize = () => appWindow.minimize();
const toggleMax = () => appWindow.toggleMaximize();
const close = () => appWindow.close();

// Manual Window Drag
const startWindowDrag = async () => {
    await appWindow.startDragging();
};
</script>

<template>
    <!-- Parent handles drag -->
    <div class="titlebar" @mousedown="startWindowDrag">

        <!-- STOP Mousedown propagation here so buttons work -->
        <div class="traffic-lights" @mousedown.stop>
            <button class="win-btn close" @click.stop="close">
                <i class="bi bi-x"></i>
            </button>
            <button class="win-btn min" @click.stop="minimize">
                <i class="bi bi-dash"></i>
            </button>
            <button class="win-btn max" @click.stop="toggleMax">
                <i class="bi bi-fullscreen"></i>
            </button>
        </div>

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
    align-items: center;
    justify-content: space-between;
    padding: 0 16px;
    z-index: 9999;
    user-select: none;
    cursor: default;
    position: relative;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.app-title {
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    font-size: 11px;
    font-weight: 800;
    letter-spacing: 2px;
    color: rgba(255, 255, 255, 0.3);
    pointer-events: none; /* Dragging works even through text */
    text-transform: uppercase;
    transition: all 0.5s ease;
}

.app-title.playing {
    color: var(--accent-primary);
    text-shadow: 0 0 10px var(--accent-primary);
}

.traffic-lights {
    display: flex;
    gap: 8px;
    z-index: 10000;
}

.win-btn {
    width: 13px;
    height: 13px;
    border-radius: 50%;
    border: none;
    padding: 0;
    display: flex;
    justify-content: center;
    align-items: center;
    color: transparent; /* Icons hidden by default */
    transition: 0.2s;
    cursor: pointer;
    font-size: 9px;
}

/* Show icons only when hovering over the group (Mac Style) */
.traffic-lights:hover .win-btn {
    color: rgba(0, 0, 0, 0.5);
}

/* Traffic Light Colors */
.win-btn.close { background: #ff5f56; }
.win-btn.min { background: #ffbd2e; }
.win-btn.max { background: #27c93f; }

.win-btn.close:hover { background: #ff7b73; }
.win-btn.min:hover { background: #ffcf66; }
.win-btn.max:hover { background: #3ef056; }

.spacer { width: 50px; }
</style>
