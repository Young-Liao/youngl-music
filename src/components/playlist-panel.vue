<script setup lang="ts">
import { playlist, currentIndex } from "../scripts/globals";
import { handleSelectionNeeded } from "../scripts/files/file-selection.ts";
import { onMounted, ref, watch } from "vue";
import { emit, listen } from "@tauri-apps/api/event";
import { addToPlayList, shuffle } from "../scripts/files/playlist.ts";
import { loadAudio } from "../scripts/playback/audio-playback.ts"; // Import loadAudio
import { invoke } from "@tauri-apps/api/core";

const props = defineProps<{ isOpen: boolean }>();
defineEmits(['close']);

// --- States ---
const playlistUpdated = ref(false);
const isFetching = ref(false);
const audioList = ref<any[]>([]);

// Selection Logic
const isSelectionMode = ref(false);
const selectedPaths = ref<Set<string>>(new Set());

// Context Menu Logic
const contextMenuIndex = ref<number | null>(null);

const openContextMenu = (e: MouseEvent, idx: number) => {
    e.preventDefault(); // In case you want to use right-click later
    e.stopPropagation();
    // If clicking the same one, close it, otherwise open new one
    contextMenuIndex.value = contextMenuIndex.value === idx ? null : idx;
};

const closeContextMenu = () => {
    contextMenuIndex.value = null;
};

// --- Actions ---

const playNow = async (idx: number) => {
    if (isSelectionMode.value) return;
    currentIndex.value = idx;
    await loadAudio(playlist.value[idx]);
    closeContextMenu();
};

const removeFromList = (idx: number) => {
    console.log("TODO: Implement Remove for index:", idx);
    // Logic: playlist.value.splice(idx, 1);
    closeContextMenu();
};

const handleItemClick = (idx: number, path: string) => {
    if (isSelectionMode.value) {
        toggleSelection(path);
    }
    // Normal click does nothing now to avoid accidental skips,
    // strictly following your 'double-click to play' request.
};

const toggleSelection = (path: string) => {
    if (selectedPaths.value.has(path)) {
        selectedPaths.value.delete(path);
    } else {
        selectedPaths.value.add(path);
    }
};

const enterSelectionMode = (initialPath?: string) => {
    isSelectionMode.value = true;
    closeContextMenu();
    if (initialPath) selectedPaths.value.add(initialPath);
};

const exitSelectionMode = () => {
    isSelectionMode.value = false;
    selectedPaths.value.clear();
};

const deleteSelected = () => {
    console.log("TODO: Implement Delete Batch:", Array.from(selectedPaths.value));
    exitSelectionMode();
};

// --- Utilities ---

const shufflePlaylist = () => {
    if (playlist.value.length <= 1) return;
    let newPlaylist = [...playlist.value];
    let newAudioList = [...audioList.value];
    shuffle(newPlaylist, newAudioList);
    playlist.value = newPlaylist;
    audioList.value = newAudioList;
};

const refreshData = async () => {
    if (playlist.value.length === 0) { audioList.value = []; return; }
    isFetching.value = true;
    playlistUpdated.value = false;
    try {
        const metadata = await invoke<any[]>('fetch_metadata', { paths: playlist.value });
        audioList.value = metadata.map((item, index) => ({
            ...item,
            path: playlist.value[index],
            title: item.title || playlist.value[index].split(/[\\/]/).pop() || "Unknown Title"
        }));
    } catch (e) { console.error(e); } finally { isFetching.value = false; }
}

onMounted(async () => {
    await listen('refresh-playlist', () => {
        props.isOpen ? refreshData() : playlistUpdated.value = true;
    });
    await listen('file-selected', async (event) => {
        const path = event.payload;
        if (Array.isArray(path)) addToPlayList(path);
        else if (typeof path === 'string') addToPlayList([path]);
        await emit("playlist-updated-successfully");
    });
});

watch(() => props.isOpen, (newVal) => {
    if (newVal && (playlistUpdated.value || audioList.value.length === 0)) refreshData();
});
</script>

<template>
    <div class="playlist-sidebar" :class="{ 'is-open': isOpen, 'selection-active': isSelectionMode }">

        <!-- BLOCKER OVERLAY: Captures clicks to close menu and prevents item interaction -->
        <div v-if="contextMenuIndex !== null" class="menu-overlay-blocker" @click.stop="closeContextMenu"></div>

        <!-- Selection Toolbar -->
        <div class="selection-toolbar" v-if="isSelectionMode">
            <button class="tool-btn" @click="exitSelectionMode"><i class="bi bi-x-lg"></i></button>
            <span>{{ selectedPaths.size }} Selected</span>
            <div class="spacer"></div>
            <button class="tool-btn delete" @click="deleteSelected" :disabled="selectedPaths.size === 0">
                <i class="bi bi-trash3"></i>
            </button>
        </div>

        <div class="sidebar-header" v-else>
            <div class="header-left">
                <div class="title-section">
                    <i class="bi bi-collection-play"></i>
                    <span>Queue</span>
                </div>
                <button v-if="playlist.length > 1" class="action-icon-btn" @click.stop="shufflePlaylist">
                    <i class="bi bi-shuffle"></i>
                </button>
            </div>
            <div class="header-right">
                <button class="action-icon-btn" @click="enterSelectionMode()" title="Select Multiple">
                    <i class="bi bi-list-check"></i>
                </button>
                <button class="close-panel-btn" @click="$emit('close')">
                    <i class="bi bi-chevron-right"></i>
                </button>
            </div>
        </div>

        <div class="sidebar-content">
            <div v-if="isFetching" class="loading-skeleton">
                <div v-for="i in 8" :key="i" class="skeleton-item"><div class="skel-thumb"></div><div class="skel-details"><div class="skel-line title"></div><div class="skel-line artist"></div></div></div>
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
                            'is-selected': selectedPaths.has(song.path),
                            'is-playing': idx === currentIndex
                        }"
                        :style="{ '--i': idx }"
                        @click="handleItemClick(idx, song.path)"
                        @dblclick="playNow(idx)"
                    >
                        <!-- Selection indicator -->
                        <div class="select-indicator" v-if="isSelectionMode">
                            <div class="circle">
                                <i class="bi bi-check-lg"></i>
                            </div>
                        </div>

                        <!-- Current playing indicator or Index -->
                        <div class="index-container" v-else>
                            <i v-if="idx === currentIndex" class="bi bi-play-fill playing-icon"></i>
                            <span v-else class="song-idx">{{ (idx + 1).toString().padStart(2, '0') }}</span>
                        </div>

                        <div class="song-thumb">
                            <img v-if="song.cover" :src="song.cover" class="thumb-img" />
                            <div v-else class="thumb-placeholder"><i class="bi bi-music-note"></i></div>
                        </div>

                        <div class="song-details">
                            <span class="song-title" :title="song.title">{{ song.title }}</span>
                            <span class="song-artist">{{ song.artist || 'Unknown Artist' }}</span>
                        </div>

                        <!-- Context Menu Trigger -->
                        <div class="song-actions" @click.stop="openContextMenu(idx)">
                            <i class="bi bi-three-dots"></i>

                            <div class="context-menu" v-if="contextMenuIndex === idx">
                                <button @click.stop="playNow(idx)">
                                    <i class="bi bi-play-circle"></i> Play Now
                                </button>
                                <button @click.stop="enterSelectionMode(song.path)">
                                    <i class="bi bi-check2-square"></i> Select
                                </button>
                                <div class="divider"></div>
                                <button class="danger" @click.stop="removeFromList(idx)">
                                    <i class="bi bi-trash"></i> Remove
                                </button>
                            </div>
                        </div>
                    </div>
                </TransitionGroup>
            </div>
        </div>

        <div class="sidebar-footer" v-if="!isSelectionMode">
            <button class="modern-add-btn" @click.stop="handleSelectionNeeded">
                <i class="bi bi-plus-circle-dotted"></i>
                <span>Import Audio</span>
            </button>
        </div>
    </div>
</template>

<style scoped src="../styles/components/playlist-panel.css"/>
