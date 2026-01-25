<script setup lang="ts">
import {playlist, currentIndex, isPaused, currentMetadata, appendedJustNow} from "../scripts/globals";
import { handleSelectionNeeded } from "../scripts/files/file-selection.ts";
import {nextTick, onMounted, onUnmounted, ref, watch} from "vue";
import { emit, listen } from "@tauri-apps/api/event";
import { addToPlayList, shuffle } from "../scripts/files/playlist.ts";
import { invoke } from "@tauri-apps/api/core";
import {loadAudio, resetStates} from "../scripts/playback/audio-playback.ts";

const props = defineProps<{ isOpen: boolean }>();
defineEmits(['close']);

const playlistUpdated = ref(false);
const isFetching = ref(false);

// --- Multi-Select State ---
const isSelectMode = ref(false);
const selectedPaths = ref(new Set<string>());
const highlightIndices = ref(new Set<number>());
const leavingIndices = ref(new Set<number>());

const audioList = ref<{
    title: string,
    artist: string,
    album: string,
    cover: string,
    total_duration: number,
    path: string
}[]>([])

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
        console.log(currentMetadata.value);
    }
};

const shufflePlaylist = () => {
    if (playlist.value.length <= 1) return;
    if (isFetching.value) return;

    let originPlay = playlist.value[currentIndex.value];

    let newPlaylist = [...playlist.value];
    let newAudioList = [...audioList.value];
    shuffle(newPlaylist, newAudioList);

    currentIndex.value = -1;

    playlist.value = newPlaylist;
    audioList.value = newAudioList;

    currentIndex.value = playlist.value.findIndex((item) => item == originPlay);
};

const deleteSelected = async () => {
    let current = playlist.value[currentIndex.value];

    // 添加确认弹窗
    const confirmed = await confirm(`Are your sure to remove ${selectedPaths.value.size} song${selectedPaths.value.size > 1 ? 's' : ''} from the playlist?`);
    if (!confirmed) return;

    playlist.value = playlist.value.filter(path => !selectedPaths.value.has(path));

    if (selectedPaths.value.has(current)) {
        invoke('stop_song').then(resetStates);
    } else {
        currentIndex.value = playlist.value.findIndex((item) => item == current);
    }
    selectedPaths.value.clear();
    isSelectMode.value = false;
    refreshData();
};

const toggleSelectAll = () => {
    if (selectedPaths.value.size === audioList.value.length) {
        selectedPaths.value.clear();
    } else {
        audioList.value.forEach(song => selectedPaths.value.add(song.path));
    }
};

const scrollToItem = (idx: number, alignTo: ScrollLogicalPosition = 'center') => {
    if (idx < 0) return;

    setTimeout(() => {
        const container = document.querySelector('.sidebar-content');
        const items = document.querySelectorAll('.song-item');
        const target = items[idx] as HTMLElement;

        if (target && container) {
            target.scrollIntoView({
                behavior: 'smooth',
                block: alignTo
            });
        }
    }, 50); // 稍微增加一点延时确保渲染
};

// --- Refresh Logic with Animation Trigger ---
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

        // === Check for Newly Appended Files ===
        if (appendedJustNow.value.length > 0) {
            await handleNewFilesEffect();
        }

    } catch (e) { console.error(e); }
    finally { isFetching.value = false; }
}

// --- Specific Logic for New Files Animation ---
const handleNewFilesEffect = async () => {
    const addedCount = appendedJustNow.value.length;
    const totalCount = audioList.value.length;

    // 计算新文件的起始索引
    const startIdx = totalCount - addedCount;
    if (startIdx < 0) return;

    // 1. 标记高亮
    // 1. 进入高亮
    highlightIndices.value.clear();
    leavingIndices.value.clear();
    for (let i = startIdx; i < totalCount; i++) {
        highlightIndices.value.add(i);
    }

    appendedJustNow.value = [];
    await nextTick();
    scrollToItem(startIdx > 0 ? startIdx - 1 : 0, 'start');

    // 2. 优雅消失逻辑
    setTimeout(() => {
        // 第一阶段：添加“正在离开”类名
        // 此时 CSS 开始执行 transition (1.2s)
        highlightIndices.value.forEach(idx => leavingIndices.value.add(idx));

        // 第二阶段：等 CSS 动画彻底走完，再移除所有类名
        // 这里的 1200ms 必须大于或等于 CSS 中的 transition 时间
        setTimeout(() => {
            highlightIndices.value.clear();
            leavingIndices.value.clear();
        }, 1200);

    }, 1750); // 1.75秒后开始退场
};

// --- New: Keyboard Focus State ---
const activeIndex = ref(-1);

/**
 * Centralized Keyboard Handler for the Playlist
 */
const handlePlaylistKeyDown = async (e: KeyboardEvent) => {
    if (!props.isOpen) return;

    // Prevent navigation if the user is typing in an input (e.g., a search bar)
    if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;

    switch (e.key) {
        case 'ArrowUp':
            e.preventDefault();
            activeIndex.value -= 1;
            activeIndex.value %= audioList.value.length;
            activeIndex.value += audioList.value.length;
            activeIndex.value %= audioList.value.length;
            scrollToItem(activeIndex.value);
            break;

        case 'ArrowDown':
            e.preventDefault();
            activeIndex.value += 1;
            activeIndex.value %= audioList.value.length;
            activeIndex.value += audioList.value.length;
            activeIndex.value %= audioList.value.length;
            scrollToItem(activeIndex.value);
            break;

        case 'Enter':
            e.preventDefault();
            if (activeIndex.value !== -1) {
                const song = audioList.value[activeIndex.value];

                // Logic branch: Toggle selection if in Select Mode, else play
                if (isSelectMode.value) {
                    if (selectedPaths.value.has(song.path)) {
                        selectedPaths.value.delete(song.path);
                    } else {
                        selectedPaths.value.add(song.path);
                    }
                } else {
                    handleItemClick(song, activeIndex.value);
                }
            }
            break;

        case 'Escape':
            if (isSelectMode.value) {
                toggleSelectMode();
            } else {
                emit('close');
            }
            break;
    }
};

// Toggle event listeners based on whether the sidebar is open
watch(() => props.isOpen, (newVal) => {
    if (newVal) {
        // Sync visual pointer with current playing song on open
        activeIndex.value = currentIndex.value;
        scrollToItem(activeIndex.value);
        window.addEventListener('keydown', handlePlaylistKeyDown);
    } else {
        window.removeEventListener('keydown', handlePlaylistKeyDown);
    }
});

onUnmounted(() => {
    window.removeEventListener('keydown', handlePlaylistKeyDown);
});

onMounted(() => {
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

                <button class="action-icon-btn" @click.stop="toggleSelectMode" :class="{active: isSelectMode}" title="Multi-Select">
                    <i :class="isSelectMode ? 'bi bi-ui-checks' : 'bi bi-list-check'"></i>
                </button>

                <button
                    v-if="isSelectMode && audioList.length > 0"
                    class="action-icon-btn"
                    @click.stop="toggleSelectAll"
                    :title="selectedPaths.size === audioList.length ? 'Deselect All' : 'Select All'"
                >
                    <i :class="selectedPaths.size === audioList.length ? 'bi bi-dash-circle-dotted' : 'bi bi-plus-circle-dotted'"></i>
                </button>
                <button v-if="playlist.length > 1 && !isSelectMode" :class="isFetching ? 'unavailable-button' : ''" class="action-icon-btn" @click.stop="shufflePlaylist" :title="!isFetching ? 'Shuffle' : 'Fetching... Can\'t shuffle'">
                    <i class="bi bi-shuffle"></i>
                </button>
            </div>
            <button class="close-panel-btn" @click.stop="$emit('close')"><i class="bi bi-chevron-right"></i></button>
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
                            'is-selected': selectedPaths.has(song.path),
                            'is-focused': activeIndex === idx,
                            'is-just-added': highlightIndices.has(idx),
                            'is-added-leaving': leavingIndices.has(idx),
                        }"
                        @click.stop="handleItemClick(song, idx)"
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
            <button v-if="isSelectMode" class="modern-add-btn delete-mode" @click.stop="deleteSelected" :disabled="selectedPaths.size === 0">
                <i class="bi bi-trash3"></i>
                <span>Remove Selected ({{ selectedPaths.size }})</span>
            </button>
            <button v-else class="modern-add-btn" @click.stop="handleSelectionNeeded">
                <i class="bi bi-plus-circle-dotted"></i>
                <span>Import Audio</span>
            </button>
        </div>
    </div>
</template>

<style scoped src="../styles/components/playlist-panel.css"/>
