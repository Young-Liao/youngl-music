import {playbackHistory} from "../globals.ts";
import {exists} from "@tauri-apps/plugin-fs";

export const getValidLastFile = async () => {
    while (playbackHistory.value.length > 0) {
        const lastFile = playbackHistory.value[playbackHistory.value.length - 1];

        // Basic check: Does the file actually exist on disk?
        if (await exists(lastFile)) {
            return lastFile;
        } else {
            console.warn(`File not found, removing from history: ${lastFile}`);
            playbackHistory.value.pop(); // Remove invalid entry
        }
    }
    return null;
}
