import {currentIndex, playbackHistory, playlist} from "../globals.ts";
import {exists} from "@tauri-apps/plugin-fs";

export const getValidLastFileFromHistory = async () => {
    while (playbackHistory.value.length > 0) {
        const lastFile = playbackHistory.value[playbackHistory.value.length - 1];

        // Basic check: Does the file actually exist on disk?
        if (await exists(lastFile) && playlist.value.includes(lastFile)) {
            currentIndex.value = playlist.value.findIndex((item) => item == lastFile);
            return lastFile;
        } else {
            console.warn(`File not found, removing from history: ${lastFile}`);
            playbackHistory.value.pop(); // Remove invalid entry
        }
    }
    return null;
}
