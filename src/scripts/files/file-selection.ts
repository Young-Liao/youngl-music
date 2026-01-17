import {open} from "@tauri-apps/plugin-dialog";
import {emit} from "@tauri-apps/api/event";
import {getValidLastFile} from "./playback-history.ts";

/// This function uses a dialog to select an audio file.
export const selectAudio = async () => {
    const file = await open({
        multiple: false,
        filters: [{
            name: 'Audio',
            extensions: ['mp3', 'wav', "flac"]
        }]
    })
    console.log("Selected file: ", file);
    return file;
};

/// This function opens the explorer and lets the user choose an song file to play.
export const handleSelectionNeeded = async () => {
    const file = await selectAudio();
    await emit('file-selected', file);
}

/// This function
export const handleFileNeeded = async () => {
    const file = await getValidLastFile();
    if (file == null) {
        console.log("Can't find a valid last file...");
        await handleSelectionNeeded()
    } else {
        console.log("Found valid last file: ", file);
        await emit('file-selected', file);
    }
}
