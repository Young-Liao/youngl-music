import {invoke} from "@tauri-apps/api/core";
import {
    currentMetadata,
    currentTime,
    isPaused,
    lockCurrentTime,
    noAudio,
    playbackHistory,
    playbackMode,
    playlist,
} from "../globals.ts";
import {startProgressCollection, stopProgressCollection} from "./progress-controller.ts";
import {listen} from "@tauri-apps/api/event";
import {handleFileNeeded} from "../files/file-selection.ts";
import {getValidLastFileFromHistory} from "../files/playback-history.ts";
import {getNextValidAudio, getPrevValidAudio} from "../files/playlist.ts";

/// Load the audio after choosing a file.
export const loadAudio = async (path: unknown) => {
    console.log("Load audio: ", path);
    if (typeof path != "string") {
        console.log("Selection Canceled... Not loading...");
        return false;
    } else {
        noAudio.value = false;
        isPaused.value = false;
        const metadata = await invoke<{
            title: string,
            artist: string,
            album: string,
            cover: string,
            total_duration: number
        }>('load_song', {path: path});
        currentMetadata.value = {
            title: metadata.title || path.split(/[\\/]/).pop() || "Unknown Title",
            artist: metadata.artist || "Unknown Artist",
            cover: metadata.cover, // This is our Base64 string
            totalDuration: metadata.total_duration,
        };
        if (!lockCurrentTime)
            currentTime.value = 0;
        startProgressCollection();
        if (path !== playbackHistory.value[playbackHistory.value.length - 1])
            playbackHistory.value.push(path as string);
        console.log("The song has been loaded.")

        await syncSystemMetadata();
        await syncPlaybackStatus();

        return true;
    }
}

/// Reset all the state to no-audio state
export const resetStates = async () => {
    stopProgressCollection()
    noAudio.value = true;
    isPaused.value = true;
    currentTime.value = 0;
    currentMetadata.value = {
        title: 'No Audio Selected',
        artist: 'Unknown Artist',
        cover: null as string | null,
        totalDuration: 0,
    };
    await checkAudioAvailability('playlist', true);
}

const getFromFile = async () => {
    return new Promise<boolean>((resolve, reject) => {
        let unlisten_suc: () => void;
        let unlisten_fai: () => void;

        const cleanup = () => {
            if (unlisten_suc) unlisten_suc();
            if (unlisten_fai) unlisten_fai();
        };

        const setup = async () => {
            unlisten_suc = await listen("playlist-updated-successfully", async () => {
                cleanup();
                resolve(true);
            });
            unlisten_fai = await listen("playlist-updated-unsuccessfully", async () => {
                cleanup();
                resolve(false);
            })
        };
        setup()
            .then(() => handleFileNeeded())
            .catch((e) => {
                cleanup();
                reject(e);
            });
    })
};

/// Check if there's an available audio to play. If not, let the user choose one, and then START PLAYING
export const checkAudioAvailability = async (priority: string = 'playlist', noHistory = false) => {
    console.log("Checking audio availability...");

    // If audio is already loaded and ready, we are good.
    if (!noAudio.value) {
        console.log("Audio already loaded.");
        return true;
    }

    let file: string | null;

    // 1. Try the primary priority
    if (priority === 'playlist') {
        file = await getNextValidAudio(playbackMode.value);
    } else {
        file = await getValidLastFileFromHistory();
    }

    // 2. Fallthrough: If the primary failed, try the alternative
    if (file == null) {
        console.log(`Priority ${priority} failed, trying alternative...`);
        if (priority === 'playlist') {
            if (!noHistory)
                file = await getValidLastFileFromHistory();
        } else {
            file = await getNextValidAudio(playbackMode.value);
        }
    }

    // 3. No invalid file: let the user choose and add to the playlist.
    if (file == null && !noHistory) {
        const res = await getFromFile();
        if (res) {
            file = await getNextValidAudio(playbackMode.value);
        }
    }

    return await loadAudio(file);
};

/// Call Rust to toggle audio playback.
export const toggleAudioPlayback = async () => {
    const origin_no_audio = noAudio.value;
    if (!await checkAudioAvailability()) {
        console.log("Rejected. Not toggling...")
    }
    if (!noAudio.value && origin_no_audio) {
        console.log("Chose one just now, not toggling...");
        return;
    }
    isPaused.value = await invoke<boolean>('toggle_playback');
    if (isPaused.value) {
        stopProgressCollection();
    } else {
        startProgressCollection();
    }
    await syncPlaybackStatus();
    console.log("State toggled. Is paused? ", isPaused.value);
}

export const skipStart = async () => {
    console.log(playlist.value);
    let file = await getPrevValidAudio();
    await loadAudio(file);
}

export const skipEnd = async () => {
    console.log(playlist.value);
    let file = await getNextValidAudio(playbackMode.value, true);
    await loadAudio(file);
}