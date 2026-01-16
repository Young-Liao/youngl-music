<script setup lang="ts">
import {onMounted} from 'vue'
import {handleSelectionNeeded} from "../scripts/files/file-selection.ts";

onMounted(async () => {
})

</script>

<template>
    <button @click="handleSelectionNeeded">Choose a Song</button>
</template>