import {currentIndex, playbackHistory, playlist} from "../globals.ts";
import { exists } from "@tauri-apps/plugin-fs";
import {emit} from "@tauri-apps/api/event";
import {getValidLastFileFromHistory} from "./playback-history.ts";

/**
 * Adds multiple file paths to the playlist.
 * Ignores files already present to prevent duplicates.
 */
export const addToPlayList = (paths: string[]) => {
    console.log("What??");
    // 1. Create a Set of current paths for high-performance lookup
    const currentSet = new Set(playlist.value);

    // 2. Filter out duplicates and invalid paths
    const newUniquePaths = paths.filter(path => path && !currentSet.has(path));

    // 3. Exit early if no new unique songs were found
    if (newUniquePaths.length === 0) return;

    // 4. Record if the playlist was empty before adding
    const wasEmpty = playlist.value.length === 0;

    // 5. Append new songs to the master list
    playlist.value.push(...newUniquePaths);

    // 6. If it was empty, ensure we point to the first newly added song
    if (wasEmpty) {
        currentIndex.value = -1;
    }

    emit('refresh-playlist').then();
};

/**
 * Shuffle an array
 */
export const shuffle = <T, T2>(array: &T[], array_: &T2[]) => {
    for (let i = array.length - 1; i > 0; i--) {
        // Pick a random index from 0 to i
        const j = Math.floor(Math.random() * (i + 1));

        // Swap elements array[i] and array[j]
        [array[i], array[j]] = [array[j], array[i]];
        [array_[i], array_[j]] = [array_[j], array_[i]];
    }
};

/**
 * Removes a file from the playlist and triggers a shuffle refresh.
 */
const removeFile = (index: number) => {
    playlist.value.splice(index, 1);

    if (playlist.value.length == 0) {
        currentIndex.value = 0;
    }
};

/**
 * Implementation with auto-cleanup and shuffle re-sync
 */
export const getNextValidAudio = async (mode: number, must: boolean = false): Promise<string | null> => {
    if (playlist.value.length === 0) return null;

    let attempts = 0;
    // We loop until we find a valid file or empty the list
    while (playlist.value.length > 0 && attempts < playlist.value.length) {
        // 1. Calculate Pointers based on Mode
        if (mode === 2 && !must) {
            // Repeat 1 logic: stay on current index unless we are forced to skip
        } else if (mode === 0) {
            currentIndex.value = Math.floor(Math.random() * playlist.value.length);
        } else {
            // Repeat All / Order
            currentIndex.value = (currentIndex.value + 1) % playlist.value.length;
        }

        const currentPath = playlist.value[currentIndex.value];

        // 2. Validate existence
        try {
            const fileExists = await exists(currentPath);
            if (fileExists) {
                return currentPath;
            } else {
                // FILE IS INVALID: Remove it and re-calculate
                removeFile(currentIndex.value);
                // After removal, the shuffle list is different.
                // We reset attempts and try again from the new "current"
                attempts = 0;
                if (mode === 2) mode = 1; // Fallback to "Repeat All" logic if Repeat 1 file is gone
            }
        } catch (err) {
            removeFile(currentIndex.value);
            attempts++;
        }
    }

    return null;
};

/**
 * To get the previous file from the history.
 */
export const getPrevValidAudio = async () => {
    if (playbackHistory.value.length == 0) return null;
    console.log(playbackHistory.value.length);
    playbackHistory.value.pop();
    let attempt_history = getValidLastFileFromHistory();
    if (attempt_history != null) {
        playbackHistory.value.pop();
        return attempt_history;
    }

    return null;
}

/**
 * Removes multiple indices from the playlist and handles index shifting
 */
export const removeMultipleFromPlaylist = (indices: number[]) => {
    // Sort descending to prevent index shifting during deletion
    const sortedIndices = [...indices].sort((a, b) => b - a);

    sortedIndices.forEach(index => {
        playlist.value.splice(index, 1);
        // Adjust currentIndex if necessary
        if (index < currentIndex.value) {
            currentIndex.value--;
        } else if (index === currentIndex.value) {
            // If we deleted the playing song, we stay at same index (which is now the next song)
            // but we might need to clamp it
            if (currentIndex.value >= playlist.value.length) {
                currentIndex.value = Math.max(0, playlist.value.length - 1);
            }
        }
    });

    emit('refresh-playlist').then();
};
