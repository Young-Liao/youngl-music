import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {currentMetadata, currentTime, isPaused} from "../globals";
import { toggleAudioPlayback, skipEnd, skipStart } from "../playback/audio-playback";

/**
 * 监听系统媒体信号（硬件按键）
 */
export const initSystemMediaListeners = async () => {
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
