import {open} from "@tauri-apps/plugin-dialog";
import {emit} from "@tauri-apps/api/event";
import {getValidLastFileFromHistory} from "./playback-history.ts";

/// This function uses a dialog to select an audio file.
export const selectAudio = async () => {
    return await open({
        multiple: true,
        filters: [{
            name: 'Audio',
            extensions: ['mp3', 'wav', "flac"]
        }]
    });
};

/// This function opens the explorer and lets the user choose an song file to play.
export const handleSelectionNeeded = async () => {
    const file = await selectAudio();
    await emit('file-selected', file);
}

/// This function
export const handleFileNeeded = async () => {
    const file = await getValidLastFileFromHistory();
    if (file == null) {
        console.log("Can't find a valid last file...");
        await handleSelectionNeeded()
    } else {
        console.log("Found valid last file: ", file);
        await emit('file-selected', file);
    }
}
