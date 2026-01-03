<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import MusicPicker from "./components/music-picker.vue";

const selected = ref<string | null>(null);

const handleSelection = (path: string) => {
    console.log("Chose: ", path);
    selected.value = path;
}

</script>

<template>
    <main class="container">
        <MusicPicker @file-selected="handleSelection" />
    </main>
</template>

<style scoped>
.logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
    filter: drop-shadow(0 0 2em #249b73);
}

</style>