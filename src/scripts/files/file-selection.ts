import {open} from "@tauri-apps/plugin-dialog";
import {emit} from "@tauri-apps/api/event";

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
