import {invoke} from "@tauri-apps/api/core"
import {currentTime, totalDuration} from "../globals.ts";

export let progressCollector: any = null;

export const updateProgress = async () => {
    // TODO auto update the progress using a timer.
    // TODO do nothing when the collector is NULL
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
