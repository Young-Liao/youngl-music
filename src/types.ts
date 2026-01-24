/** 
 * Common type definitions for the application 
 */

export interface LyricLine {
    time: number;   // Seconds (f64 in Rust)
    text: string;
}

export interface SongMetadata {
    title: string;
    artist: string;
    album?: string;
    cover: string | null;
    totalDuration: number;
    lyrics?: LyricLine[];
}

export enum PlaybackMode {
    Shuffle = 0,
    RepeatAll = 1,
    RepeatOne = 2
}