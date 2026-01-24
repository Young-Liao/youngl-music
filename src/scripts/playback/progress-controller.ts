import {invoke} from "@tauri-apps/api/core"
import {currentMetadata, currentTime} from "../globals.ts";
import {syncPlaybackStatus} from "../utils/system-api.ts";

export let progressController: number | null = null;

export const updateProgress = async () => {
    try {
        currentTime.value = await invoke<number>('fetch_progress');
        await syncPlaybackStatus();
    } catch (e) {
        console.log("Failed to fetch progress: ", e);
    }
}

export const stopProgressCollection = () => {
    if (progressController) {
        clearInterval(progressController);
        progressController = null;
    }
}

export const startProgressCollection = () => {
    stopProgressCollection();
    progressController = setInterval(updateProgress, 250);
}

export const setPosition = async (time: number) => {
    try {
        await invoke('set_position', { time: time });
    } catch (e) {
        console.log("Failed to set position: ", e);
    }
}

export const seekForward = async (time: number) => {
    await setPosition(Math.min(currentTime.value + time, currentMetadata.value.totalDuration));
}

export const seekBackward = async (time: number) => {
    await setPosition(Math.max(currentTime.value - time, 0));
}
