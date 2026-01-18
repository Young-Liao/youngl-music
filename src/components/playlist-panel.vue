<!-- src/components/playlist-panel.vue -->
<script setup lang="ts">
import { playlist, currentIndex, isPaused } from "../scripts/globals";
import { handleSelectionNeeded } from "../scripts/files/file-selection.ts";
import { onMounted, ref, watch, onUnmounted } from "vue";
import {emit, listen} from "@tauri-apps/api/event";
import { addToPlayList, shuffle } from "../scripts/files/playlist.ts";
import { invoke } from "@tauri-apps/api/core";
import { loadAudio } from "../scripts/playback/audio-playback.ts";

const props = defineProps<{ isOpen: boolean }>();
defineEmits(['close']);

const playlistUpdated = ref(false);
const isFetching = ref(false);

// --- Multi-Select State ---
const isSelectMode = ref(false);
const selectedPaths = ref(new Set<string>());

// --- Context Menu State ---
const showMenu = ref(false);
const menuPos = ref({ x: 0, y: 0 });
const menuTargetIdx = ref(-1);

const audioList = ref<{
    title: string,
    artist: string,
    album: string,
    cover: string,
    total_duration: number,
    path: string
}[]>([])

// Logic to check if a song is currently the one loaded
const isActive = (path: string) => {
    return playlist.value[currentIndex.value] === path;
};

const toggleSelectMode = () => {
    isSelectMode.value = !isSelectMode.value;
    selectedPaths.value.clear();
};

const handleItemClick = async (song: any, idx: number) => {
    if (isSelectMode.value) {
        if (selectedPaths.value.has(song.path)) selectedPaths.value.delete(song.path);
        else selectedPaths.value.add(song.path);
    } else {
        currentIndex.value = idx;
        await loadAudio(song.path);
    }
};

const openContextMenu = (e: MouseEvent, idx: number) => {
    e.preventDefault();
    if (isSelectMode.value) return; // Disable context menu in select mode
    menuTargetIdx.value = idx;
    menuPos.value = { x: e.clientX, y: e.clientY };
    showMenu.value = true;
};

const closeMenu = () => { showMenu.value = false; };

const shufflePlaylist = () => {
    if (playlist.value.length <= 1) return;

    let newPlaylist = [...playlist.value];
    let newAudioList = [...audioList.value];

    shuffle(newPlaylist, newAudioList);

    playlist.value = newPlaylist;
    audioList.value = newAudioList;
};

const playNow = async () => {
    currentIndex.value = menuTargetIdx.value;
    await loadAudio(playlist.value[menuTargetIdx.value]);
    closeMenu();
};

const deleteFromQueue = () => {
    playlist.value.splice(menuTargetIdx.value, 1);
    // Adjust index if we deleted something before the current playing song
    if (menuTargetIdx.value < currentIndex.value) currentIndex.value--;
    refreshData();
    closeMenu();
};

const deleteSelected = () => {
    playlist.value = playlist.value.filter(path => !selectedPaths.value.has(path));
    selectedPaths.value.clear();
    isSelectMode.value = false;
    refreshData();
};

const refreshData = async () => {
    if (playlist.value.length === 0) {
        audioList.value = [];
        return;
    }
    isFetching.value = true;
    try {
        const metadata = await invoke<any[]>('fetch_metadata', { paths: playlist.value });
        audioList.value = metadata.map((item, index) => ({
            ...item,
            path: playlist.value[index],
            title: item.title || playlist.value[index].split(/[\\/]/).pop() || "Unknown Title"
        }));
    } catch (e) { console.error(e); }
    finally { isFetching.value = false; }
}

onMounted(() => {
    window.addEventListener('click', closeMenu);
    listen('refresh-playlist', () => props.isOpen ? refreshData() : playlistUpdated.value = true);
    listen('file-selected', async (event) => {
        const path = event.payload;
        if (Array.isArray(path)) {
            addToPlayList(path);
            await emit("playlist-updated-successfully");
        } else {
            if (typeof path === 'string') addToPlayList([path]);
            await emit("playlist-updated-successfully");
        }
    });
});
onUnmounted(() => window.removeEventListener('click', closeMenu));

watch(() => props.isOpen, (newVal) => {
    if (newVal && (playlistUpdated.value || audioList.value.length === 0)) refreshData();
});
</script>

<template>
    <div class="playlist-sidebar" :class="{ 'is-open': isOpen }">
        <div class="sidebar-header">
            <div class="header-left">
                <div class="title-section">
                    <i class="bi bi-collection-play"></i>
                    <span>Queue</span>
                </div>
                <button class="action-icon-btn" @click="toggleSelectMode" :class="{active: isSelectMode}" title="Multi-Select">
                    <i class="bi bi-check2-all"></i>
                </button>
                <button v-if="playlist.length > 1 && !isSelectMode" class="action-icon-btn" @click="shufflePlaylist" title="Shuffle">
                    <i class="bi bi-shuffle"></i>
                </button>
            </div>
            <button class="close-panel-btn" @click="$emit('close')"><i class="bi bi-chevron-right"></i></button>
        </div>

        <div class="sidebar-content">
            <div v-if="isFetching" class="loading-skeleton">
                <div v-for="i in 8" :key="i" class="skeleton-item"><div class="skel-thumb"></div></div>
            </div>

            <div v-else-if="playlist.length === 0" class="empty-list">
                <i class="bi bi-music-note-list"></i>
                <p>No songs in queue</p>
            </div>

            <div v-else class="song-list-container">
                <TransitionGroup name="playlist-stagger">
                    <div
                        v-for="(song, idx) in audioList"
                        :key="song.path"
                        class="song-item"
                        :class="{
                            'is-current': isActive(song.path),
                            'is-playing': isActive(song.path) && !isPaused,
                            'is-selected': selectedPaths.has(song.path)
                        }"
                        @click="handleItemClick(song, idx)"
                        @contextmenu="openContextMenu($event, idx)"
                        :style="{ '--i': idx }"
                    >
                        <div v-if="isSelectMode" class="select-indicator">
                            <i :class="selectedPaths.has(song.path) ? 'bi bi-check-circle-fill' : 'bi bi-circle'"></i>
                        </div>
                        <span v-else class="song-idx">{{ (idx + 1).toString().padStart(2, '0') }}</span>

                        <div class="song-thumb">
                            <img v-if="song.cover" :src="song.cover" class="thumb-img" alt="cover" />
                            <div v-else class="thumb-placeholder"><i class="bi bi-music-note"></i></div>
                        </div>

                        <div class="song-details">
                            <span class="song-title">{{ song.title }}</span>
                            <span class="song-artist">{{ song.artist || 'Unknown Artist' }}</span>
                        </div>
                    </div>
                </TransitionGroup>
            </div>
        </div>

        <div class="sidebar-footer">
            <button v-if="isSelectMode" class="modern-add-btn delete-mode" @click="deleteSelected" :disabled="selectedPaths.size === 0">
                <i class="bi bi-trash3"></i>
                <span>Remove Selected ({{ selectedPaths.size }})</span>
            </button>
            <button v-else class="modern-add-btn" @click.stop="handleSelectionNeeded">
                <i class="bi bi-plus-circle-dotted"></i>
                <span>Import Audio</span>
            </button>
        </div>

        <!-- Context Menu -->
        <div v-if="showMenu" class="context-menu" :style="{ top: menuPos.y + 'px', left: menuPos.x + 'px' }" @click.stop>
            <div class="menu-item" @click="playNow">
                <i class="bi bi-play-fill"></i> Play Now
            </div>
            <div class="menu-divider"></div>
            <div class="menu-item danger" @click="deleteFromQueue">
                <i class="bi bi-trash3"></i> Remove
            </div>
        </div>
    </div>
</template>

<style scoped src="../styles/components/playlist-panel.css"/>
