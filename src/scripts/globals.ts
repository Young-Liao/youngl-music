import { ref } from 'vue'

// Constants
export const callWait = 50; // ms

/// Locks
export const lockCurrentTime = ref(false);

/// States
export const isPaused = ref(true);
export const currentTime = ref(0);
export const noAudio = ref(true);
export const volume = ref(80.); // 0-100

export const playbackHistory = ref<string[]>([]);

export const playbackMode = ref(0);
export const playlist = ref<string[]>([]); // The new global playlist
export const currentIndex = ref(0);

/// Data
export const currentMetadata = ref({
    title: 'No Audio Selected',
    artist: 'Unknown Artist',
    cover: null as string | null,
    totalDuration: 0,
});
