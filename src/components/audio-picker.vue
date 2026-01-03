<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';

const emit = defineEmits(['fileSelected'])

/// This function opens the explorer and lets the user to choose an audio file to play.
async function selectAudio() {
    const files = await open({
        multiple: false,
        filters: [{
            name: 'Audio',
            extensions: ['mp3', 'wav', "flac"]
        }]
    })
    console.log("Selected files: ", files);
    emit('fileSelected', files);
}

</script>

<template>
<button @click="selectAudio">Choose a Song</button>
</template>