import { ref } from 'vue'

/// States
export const isPaused = ref(true);
export const currentTime = ref(0);
export const totalDuration = ref(0);
export const noAudio = ref(true);
export const playbackHistory = ref<string[]>([]);
