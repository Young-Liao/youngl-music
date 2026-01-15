<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import {isPaused, noAudio, playbackHistory} from "../scripts/globals";
import { startProgressCollection, stopProgressCollection } from "../scripts/progress/progress-collector.ts";
import {emit, listen} from "@tauri-apps/api/event";

/// This function asks Rust to play the audio
export const handleSongPlayback = async (path: string) => {
}

export const checkAudio = async (): Promise<boolean> => {
}

/// This function asks Rust to play/pause the audio
export const handleToggle = async () => {
}

</script>