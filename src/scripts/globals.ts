import { ref } from 'vue'

/// Locks
export const lockCurrentTime = ref(false);

/// States
export const isPaused = ref(true);
export const currentTime = ref(0);
export const totalDuration = ref(0);
export const noAudio = ref(true);
export const volume = ref(80); // 0-100
export const playbackHistory = ref<string[]>([]);
export const playlist = ref<string[]>([]); // The new global playlist
