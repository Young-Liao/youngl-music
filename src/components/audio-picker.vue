<script setup lang="ts">
import {onMounted} from 'vue'
import {selectAudio} from "../scripts/files/file-selection.ts";
import {emit} from "@tauri-apps/api/event";


/// This function opens the explorer and lets the user choose an audio file to play.
const handleSelectionNeeded = async () => {
    const file = await selectAudio();
    await emit('file-selected', file);
}

onMounted(async () => {
})

</script>

<template>
    <button @click="handleSelectionNeeded">Choose a Song</button>
</template>