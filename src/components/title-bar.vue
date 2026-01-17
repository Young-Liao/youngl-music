<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window';
import { isPaused } from '../scripts/globals';

const minimize = () => getCurrentWindow().minimize();
const toggleMax = () => getCurrentWindow().toggleMaximize();
const close = () => getCurrentWindow().close();
</script>

<template>
    <div class="titlebar" data-tauri-drag-region>
        <div class="traffic-lights">
            <button class="win-btn close" @click="close"><i class="bi bi-x-lg"></i></button>
            <button class="win-btn min" @click="minimize"><i class="bi bi-dash-lg"></i></button>
            <button class="win-btn max" @click="toggleMax"><i class="bi bi-arrows-angle-expand"></i></button>
        </div>

        <div
            class="app-title"
            data-tauri-drag-region
            :class="{ 'playing': !isPaused }"
        >
            YoungL Music
        </div>

        <div class="spacer"></div>
    </div>
</template>

<style scoped>
/* Keep your previous CSS, just update .app-title */
.titlebar {
    height: 40px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 16px;
    z-index: 50;
    /* Glass border at bottom */
    border-bottom: 1px solid rgba(255,255,255,0.05);
}

.app-title {
    font-size: 11px;
    font-weight: 800;
    letter-spacing: 1px;
    color: rgba(255, 255, 255, 0.3); /* Default Dim */
    transition: color 0.5s ease, text-shadow 0.5s ease;
    pointer-events: none;
    text-transform: uppercase;
}

.app-title.playing {
    color: var(--accent-primary);
    text-shadow: 0 0 10px var(--accent-primary);
}

/* ... Buttons CSS from previous step ... */
.traffic-lights { display: flex; gap: 8px; }
.win-btn { width: 12px; height: 12px; border-radius: 50%; border:none; padding:0; display: flex; justify-content:center; align-items:center; color: transparent; transition: 0.2s; background: rgba(255,255,255,0.15);}
.win-btn:hover { color: rgba(0,0,0,0.6); }
.win-btn.close:hover { background: #ff5f56; }
.win-btn.min:hover { background: #ffbd2e; }
.win-btn.max:hover { background: #27c93f; }
.spacer { width: 50px; }
</style>
