/** 
 * Common type definitions for the application 
 */

export interface SongMetadata {
    title: string;
    artist: string;
    album?: string;
    cover: string | null;
    totalDuration: number;
}

export enum PlaybackMode {
    Shuffle = 0,
    RepeatAll = 1,
    RepeatOne = 2
}