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

const audioList = ref<{
    title: string,
    artist: string,
    album: string,
    cover: string,
    total_duration: number
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
    playlistUpdated.value = false;
    const metadata = await invoke<{
        title: string,
        artist: string,
        album: string,
        cover: string,
        total_duration: number
    }[]>('fetch_metadata', { paths: playlist.value });

    // Process titles
    audioList.value = metadata.map((item, index) => ({
        ...item,
        title: item.title || playlist.value[index].split(/[\\/]/).pop() || "Unknown Title"
    }));
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

watch(() => props.isOpen, () => {
    if (playlistUpdated) {
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
            <div v-if="playlist.length === 0" class="empty-list">
                <i class="bi bi-music-note-list"></i>
                <p>No songs in queue</p>
            </div>

            <div v-else class="song-list">
                <div v-for="(song, idx) in audioList" :key="idx" class="song-item">
                    <span class="song-idx">{{ (idx + 1).toString().padStart(2, '0') }}</span>

                    <!-- SONG COVER THUMBNAIL -->
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
