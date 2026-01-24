import {currentMetadata} from "../globals.ts";
import {invoke} from "@tauri-apps/api/core";
import {LyricLine} from "../../types.ts";

export const fetchLyricsFromSong = async (path: string) => {
    currentMetadata.value.lyrics = await invoke<LyricLine[]>("load_lyrics_from_song", { path: path });
}