import {throttle} from "../utils/throttling.ts";
import {invoke} from "@tauri-apps/api/core";
import {callWait, volume} from "../globals.ts";
import {ref, watch} from "vue";

const lastVolume = ref(volume.value || 80); // Store volume for unmuting

export const toggleMute = () => {
    if (volume.value > 0) {
        lastVolume.value = volume.value;
        volume.value = 0;
    } else {
        volume.value = lastVolume.value || 80;
    }
};

const updateVolume = throttle(async () => {
    await invoke("set_volume", { volume: volume.value});
}, callWait);

export const volumeUp = (val: number) => {
    volume.value += Math.min(val, 100 - volume.value);
}

export const volumeDown = (val: number) => {
    volume.value -= Math.min(val, volume.value);
}

watch(volume, (newValue) => {
    updateVolume(newValue);
})
