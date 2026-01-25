import { invoke } from "@tauri-apps/api/core";
import {emit, listen} from "@tauri-apps/api/event";
import {currentIndex, currentMetadata, currentTime, isPaused, playlist, playlistCloseTimer} from "../globals";
import {toggleAudioPlayback, skipEnd, skipStart, resetStates} from "../playback/audio-playback";
import {addToPlayList} from "../files/playlist.ts";

const handleMusicFiles = (filePaths: string[]) => {
    if (filePaths.length > 0) {
        let newIndex = playlist.value.length;
        addToPlayList(filePaths);
        currentIndex.value = newIndex - 1;
        resetStates().then(() => emit("open-menu"));
    }
}

export const cancelPendingClose = () => {
    if (playlistCloseTimer.value) {
        clearTimeout(playlistCloseTimer.value);
        playlistCloseTimer.value = null;
    }
};

/**
 * 监听系统媒体信号（硬件按键）
 */
export const initSystemListeners = async () => {
    try {
        const initialFiles = await invoke<string[]>("get_start_args");
        if (initialFiles && initialFiles.length > 0) {
            handleMusicFiles(initialFiles);
        }
    } catch (err) {
        console.error("Failed to fetch the start args:", err);
    }
    await listen<string>("system-media-event", (event) => {
        const signal = event.payload;
        if (signal === "Toggle") {
            toggleAudioPlayback();
        } else if (signal === "Next") {
            skipEnd();
        } else if (signal === "Previous") {
            skipStart();
        }
    });
    await listen<string[]>('open-file', (event) => {
        const filePaths = event.payload;
        handleMusicFiles(filePaths);
    });
}

/**
 * 更新系统媒体元数据（歌名、作者等）
 */
export const syncSystemMetadata = async () => {
    try {
        const meta = currentMetadata.value;
        await invoke("update_system_metadata", {
            title: meta.title || "Unknown",
            artist: meta.artist || "Unknown",
            album: "YoungL Music",
            cover: meta.cover // 如果你有封面路径，可以在这里传入
        });
    } catch (e) {
        console.error("Sync Metadata Failed:", e);
    }
}

/**
 * 更新播放状态（播放/暂停按钮同步）
 */
export const syncPlaybackStatus = async () => {
    try {
        await invoke("update_system_status", {
            isPaused: isPaused.value,
            progress: currentTime.value,
        });
    } catch (e) {
        console.error("Sync Status Failed:", e);
    }
}

