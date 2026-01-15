import {open} from "@tauri-apps/plugin-dialog";

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
