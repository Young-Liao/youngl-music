import {skipEnd, skipStart, toggleAudioPlayback} from "../playback/audio-playback.ts";
import {seekBackward, seekForward} from "../playback/progress-controller.ts";
import {toggleMute, volumeDown, volumeUp} from "../controls/volume.ts";
import {isPlaylistOpen} from "../globals.ts";

const handleKeyDown = async (e: KeyboardEvent) => {
    if (["INPUT", "TEXTAREA"].includes((e.target as HTMLElement).tagName)) return;

    const { key, ctrlKey } = e;

    switch (key) {
        case "Tab":
            e.preventDefault(); // Disables the focus move
            e.stopPropagation();
            break;
        case " ":
            e.preventDefault();
            await toggleAudioPlayback();
            break;

        case "ArrowLeft":
            await seekBackward(5);
            break;

        case "ArrowRight":
            await seekForward(5);
            break;

        case 'ArrowUp':
            e.preventDefault()
            if (!isPlaylistOpen.value)
                volumeUp(5);
            break;

        case 'ArrowDown':
            e.preventDefault()
            if (!isPlaylistOpen.value)
                volumeDown(5);
            break;

        case "MediaTrackPrevious":
        case "PageUp":
            if (ctrlKey || key === "PageUp") {
                e.preventDefault();
                await skipStart();
            }
            break;

        case "MediaTrackNext":
        case "PageDown":
            if (ctrlKey || key === "PageDown") {
                e.preventDefault();
                await skipEnd();
            }
            break;

        case "l":
        case "L":
            isPlaylistOpen.value = !isPlaylistOpen.value;
            break;

        case "m":
        case "M":
            toggleMute();
            break;
    }
};

export const initKeyboardEventHandler = () => {
    window.addEventListener("keydown", handleKeyDown);
};

export const destroyKeyboardEventHandler = () => {
    window.removeEventListener("keydown", handleKeyDown);
};