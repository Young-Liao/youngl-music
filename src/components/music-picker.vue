<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';

const emit = defineEmits(['fileSelected'])

async function selectMusic() {
    const files = await open({
        multiple: true,
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
    <button @click="selectMusic">Choose a Song</button>
</template>