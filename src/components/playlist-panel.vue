<script setup lang="ts">
import {playlist} from "../scripts/globals";
import {handleSelectionNeeded} from "../scripts/files/file-selection.ts";
import {onMounted, ref, watch} from "vue";
import {emit, listen} from "@tauri-apps/api/event";
import {addToPlayList, shuffle} from "../scripts/files/playlist.ts";
import {invoke} from "@tauri-apps/api/core";

const props = defineProps<{ isOpen: boolean }>();
defineEmits(['close']);

const playlistUpdated = ref(false);
const isFetching = ref(false); // New: Loading state

const audioList = ref<{
    title: string,
    artist: string,
    album: string,
    cover: string,
    total_duration: number,
    path: string // Added to ensure stable keys for animation
}[]>([])

const shufflePlaylist = () => {
    if (playlist.value.length <= 1) return;

    let newPlaylist = [...playlist.value];
    let newAudioList = [...audioList.value];

    shuffle(newPlaylist, newAudioList);

    playlist.value = newPlaylist;
    audioList.value = newAudioList;
};

const refreshData = async () => {
    if (playlist.value.length === 0) {
        audioList.value = [];
        return;
    }

    isFetching.value = true; // Start Loading
    playlistUpdated.value = false;

    try {
        const metadata = await invoke<{
            title: string,
            artist: string,
            album: string,
            cover: string,
            total_duration: number
        }[]>('fetch_metadata', { paths: playlist.value });

        audioList.value = metadata.map((item, index) => ({
            ...item,
            path: playlist.value[index], // Crucial for TransitionGroup keys
            title: item.title || playlist.value[index].split(/[\\/]/).pop() || "Unknown Title"
        }));
    } catch (e) {
        console.error(e);
    } finally {
        isFetching.value = false; // End Loading
    }
}

onMounted(async () => {
    await listen('refresh-playlist', () => {
        if (props.isOpen) {
            refreshData();
        } else {
            playlistUpdated.value = true;
        }
    });
});

const addFiles = async () => {
    await handleSelectionNeeded();
};

watch(() => props.isOpen, (newVal) => {
    if (newVal && (playlistUpdated.value || audioList.value.length === 0)) {
        refreshData();
    }
})

onMounted(async () => {
    await listen('file-selected', async (event) => {
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
</script>

<template>
    <div class="playlist-sidebar" :class="{ 'is-open': isOpen }">
        <div class="sidebar-header">
            <div class="header-left">
                <div class="title-section">
                    <i class="bi bi-collection-play"></i>
                    <span>Queue</span>
                </div>
                <button
                    v-if="playlist.length > 1"
                    class="action-icon-btn"
                    @click.stop="shufflePlaylist"
                    title="Shuffle Queue"
                >
                    <i class="bi bi-shuffle"></i>
                </button>
            </div>

            <button class="close-panel-btn" @click="$emit('close')">
                <i class="bi bi-chevron-right"></i>
            </button>
        </div>

        <div class="sidebar-content">
            <!-- 1. LOADING ANIMATION (Skeleton) -->
            <div v-if="isFetching" class="loading-skeleton">
                <div v-for="i in 8" :key="i" class="skeleton-item">
                    <div class="skel-thumb"></div>
                    <div class="skel-details">
                        <div class="skel-line title"></div>
                        <div class="skel-line artist"></div>
                    </div>
                </div>
            </div>

            <!-- EMPTY STATE -->
            <div v-else-if="playlist.length === 0" class="empty-list">
                <i class="bi bi-music-note-list"></i>
                <p>No songs in queue</p>
            </div>

            <!-- 2. LIST ANIMATION (Transition Group) -->
            <div v-else class="song-list-container">
                <TransitionGroup name="playlist-stagger">
                    <div
                        v-for="(song, idx) in audioList"
                        :key="song.path"
                        class="song-item"
                        :style="{ '--i': idx }"
                    >
                        <span class="song-idx">{{ (idx + 1).toString().padStart(2, '0') }}</span>

                        <div class="song-thumb">
                            <img v-if="song.cover" :src="song.cover" class="thumb-img" alt="cover" />
                            <div v-else class="thumb-placeholder">
                                <i class="bi bi-music-note"></i>
                            </div>
                        </div>

                        <div class="song-details">
                            <span class="song-title" :title="song.title">{{ song.title }}</span>
                            <span class="song-artist">{{ song.artist || 'Unknown Artist' }}</span>
                        </div>

                        <div class="song-actions">
                            <i class="bi bi-three-dots"></i>
                        </div>
                    </div>
                </TransitionGroup>
            </div>
        </div>

        <div class="sidebar-footer">
            <button class="modern-add-btn" @click.stop="addFiles">
                <i class="bi bi-plus-circle-dotted"></i>
                <span>Import Audio</span>
            </button>
        </div>
    </div>
</template>

<style scoped src="../styles/components/playlist-panel.css"/>
