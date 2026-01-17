<!-- src/components/playlist-panel.vue -->
<script setup lang="ts">
import { playlist } from "../scripts/globals";
import { handleSelectionNeeded } from "../scripts/files/file-selection.ts";

defineProps<{ isOpen: boolean }>();
defineEmits(['close']);

const getFileName = (path: string) => path.split(/[\\/]/).pop() || path;

const addFiles = async () => {
    // This will trigger the file picker
    await handleSelectionNeeded();
};
</script>

<template>
    <div class="playlist-sidebar" :class="{ 'is-open': isOpen }">
        <div class="sidebar-header">
            <div class="title-section">
                <i class="bi bi-collection-play"></i>
                <span>Queue</span>
            </div>
            <button class="close-panel-btn" @click="$emit('close')">
                <i class="bi bi-chevron-right"></i>
            </button>
        </div>

        <div class="sidebar-content">
            <div v-if="playlist.length === 0" class="empty-list">
                <p>No songs in queue</p>
            </div>
            <div v-else class="song-list">
                <div v-for="(path, idx) in playlist" :key="idx" class="song-item">
                    <span class="song-idx">{{ idx + 1 }}</span>
                    <span class="song-name" :title="path">{{ getFileName(path) }}</span>
                </div>
            </div>
        </div>

        <div class="sidebar-footer">
            <button class="modern-add-btn" @click="addFiles">
                <i class="bi bi-plus-circle-dotted"></i>
                <span>Import Audio</span>
            </button>
        </div>
    </div>
</template>

<style scoped>
.playlist-sidebar {
    position: fixed;
    top: 0;
    right: -320px;
    width: 300px;
    height: 100vh;
    background: rgba(10, 10, 10, 0.7);
    backdrop-filter: blur(40px) saturate(180%);
    border-left: 1px solid rgba(255, 255, 255, 0.1);
    z-index: 10001; /* Above everything */
    transition: transform 0.5s cubic-bezier(0.16, 1, 0.3, 1);
    display: flex;
    flex-direction: column;
}

.playlist-sidebar.is-open {
    transform: translateX(-320px);
    box-shadow: -20px 0 60px rgba(0, 0, 0, 0.8);
}

.sidebar-header {
    padding: 30px 20px;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.title-section {
    display: flex;
    align-items: center;
    gap: 12px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 2px;
    font-size: 0.9rem;
    color: var(--accent-primary);
}

.close-panel-btn {
    background: transparent;
    border: none;
    color: white;
    font-size: 1.2rem;
    cursor: pointer;
    opacity: 0.5;
}

.sidebar-content {
    flex: 1;
    overflow-y: auto;
    padding: 0 15px;
}

.song-item {
    display: flex;
    align-items: center;
    padding: 12px;
    border-radius: 12px;
    margin-bottom: 5px;
    background: rgba(255, 255, 255, 0.03);
    transition: 0.2s;
    cursor: pointer;
}

.song-item:hover { background: rgba(255, 255, 255, 0.08); }

.song-idx { width: 30px; opacity: 0.3; font-family: var(--font-mono); font-size: 0.8rem; }
.song-name { flex: 1; font-size: 0.85rem; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }

.sidebar-footer { padding: 25px; }

.modern-add-btn {
    width: 100%;
    padding: 15px;
    border-radius: 16px;
    border: 1px dashed rgba(255, 255, 255, 0.2);
    background: rgba(255, 255, 255, 0.05);
    color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    cursor: pointer;
    transition: all 0.3s;
}

.modern-add-btn:hover {
    background: rgba(255, 255, 255, 0.1);
    border-color: var(--accent-primary);
    transform: translateY(-2px);
}
</style>
