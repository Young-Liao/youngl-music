import {invoke} from "@tauri-apps/api/core"
import {currentTime, totalDuration} from "./globals.ts";

export let progressCollector: any = null;

export const updateProgress = async () => {
    if (!progressCollector) return;
    try {
        const stats = await invoke<{ position: number, duration: number }>('get_playback_progress');
        currentTime.value = stats.position;
        totalDuration.value = stats.duration;
    } catch (e) {
        stopProgressCollection();
    }
}

export const stopProgressCollection = () => {
    if (progressCollector) {
        clearInterval(progressCollector);
        progressCollector = null;
    }
}

export const startProgressCollection = () => {
    stopProgressCollection();
    progressCollector = setInterval(updateProgress, 250);
}
