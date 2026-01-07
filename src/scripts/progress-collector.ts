import { invoke } from "@tauri-apps/api/core"
import { currentTime, totalDuration } from "./globals.ts";

let progressCollector: any = null;

export async function updateProgress() {
    if (!progressCollector) return;
    try {
        const stats = await invoke<{position: number, duration: number}>('get_playback_progress'); 
        currentTime.value = stats.position;
        totalDuration.value = stats.duration;
    } catch (e) {
        stopProgressCollection();
    }
}

export function stopProgressCollection() {
    if (progressCollector) {
        clearInterval(progressCollector);
        progressCollector = null;
    }
}

export function startProgressCollection() {
    stopProgressCollection();
    progressCollector = setInterval(updateProgress, 500);
}
