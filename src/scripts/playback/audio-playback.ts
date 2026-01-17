import {invoke} from "@tauri-apps/api/core";
import {currentTime, isPaused, lockCurrentTime, noAudio, playbackHistory, totalDuration} from "../globals.ts";
import {startProgressCollection, stopProgressCollection} from "./progress-controller.ts";
import {emit, listen} from "@tauri-apps/api/event";
import {handleFileNeeded} from "../files/file-selection.ts";

/// Load the audio after choosing a file.
export const loadAudio = async (path: unknown) => {
    if (path == null) {
        console.log("Selection Canceled... Not loading...");
        await emit("audio-not-loaded");
    } else {
        noAudio.value = false;
        isPaused.value = false;
        totalDuration.value = await invoke<number>('load_song', {path: path});
        if (!lockCurrentTime)
            currentTime.value = 0;
        playbackHistory.value.push(path as string);
        await emit("audio-loaded");
        console.log("The song has been loaded.")
    }
}

/// Reset all the state to no-audio state
export const resetStates = () => {
    stopProgressCollection()
    noAudio.value = true;
    isPaused.value = true;
    currentTime.value = 0;
}

/// Check if there's an available audio to play. If not, let the user choose one, and then START PLAYING
export const checkAudioAvailability = async () => {
    console.log("Checking audio availability...");
    let result = false;
    if (noAudio.value) {
        console.log("No audio loaded...");
        try {
            const res = await new Promise<{ok: boolean, data: unknown}>((resolve, reject) => {
                let unlisten_suc: () => void;
                let unlisten_fai: () => void;

                const cleanup = () => {
                    if (unlisten_suc) unlisten_suc();
                    if (unlisten_fai) unlisten_fai();
                };

                // 1. Setup listeners
                const setupListeners = async () => {
                    unlisten_suc = await listen("audio-loaded", (event) => {
                        cleanup();
                        resolve({ok: true, data: event.payload});
                    });

                    unlisten_fai = await listen("audio-not-loaded", (event) => {
                        cleanup();
                        resolve({ok: false, data: event.payload});
                    });
                };

                // 2. Run setup and then the trigger
                setupListeners()
                    .then(() => handleFileNeeded())
                    .catch((e) => {
                        cleanup();
                        reject(e);
                    });
            });
            result = res.ok;
        } catch (e) {
            console.log("Error when checking audio availability: ", e);
        }
    } else {
        result = true;
    }
    console.log("Result = ", result);
    return result;
}

/// Call Rust to toggle audio playback.
export const toggleAudioPlayback = async () => {
    if (!await checkAudioAvailability()) {
        console.log("Rejected. Not toggling...")
    }
    isPaused.value = await invoke<boolean>('toggle_playback');
    if (isPaused.value) {
        stopProgressCollection();
    } else {
        startProgressCollection();
    }
    console.log("State toggled. Is paused? ", isPaused.value);
}
