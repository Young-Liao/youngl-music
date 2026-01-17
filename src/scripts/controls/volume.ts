import {throttle} from "../utils/throttling.ts";
import {invoke} from "@tauri-apps/api/core";
import {callWait, volume} from "../globals.ts";
import {watch} from "vue";

const updateVolume = throttle(async () => {
    await invoke("set_volume", { volume: volume.value});
}, callWait);

watch(volume, (newValue) => {
    updateVolume(newValue);
})
