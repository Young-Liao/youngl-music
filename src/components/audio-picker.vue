<script setup lang="ts">
import {open} from '@tauri-apps/plugin-dialog';
import {emit, listen} from '@tauri-apps/api/event';
import {onMounted} from 'vue'
import {getValidLastFile} from "../scripts/files/playback-history.ts";

const vueEmit = defineEmits(['fileSelected'])

/// This function opens the explorer and lets the user choose an audio file to play.
async function selectAudio() {
    const file = await open({
        multiple: false,
        filters: [{
            name: 'Audio',
            extensions: ['mp3', 'wav', "flac"]
        }]
    })
    console.log("Selected files: ", file);
    vueEmit('fileSelected', file);
}

onMounted(async () => {
    await listen('need-file-selection', async () => {
        console.log("Received: need-file-selection");
        const file = await getValidLastFile();
        if (!file) await selectAudio();
        else vueEmit('fileSelected', file);
        await emit("file-selection-result");
    })
})

</script>

<template>
    <button @click="selectAudio">Choose a Song</button>
</template>