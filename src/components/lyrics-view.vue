<script setup lang="ts">
import { ref, watch, nextTick, onUnmounted } from 'vue';
import { currentMetadata, currentTime } from '../scripts/globals';
import { setPosition } from "../scripts/playback/progress-controller.ts";

const scrollContainer = ref<HTMLElement | null>(null);
const lyricElements = ref<HTMLElement[]>([]);

const activeIndex = ref(-1);
const userHoverIndex = ref(-1);
const isUserScrolling = ref(false);
const isSeeking = ref(false); // New flag to prevent snap-back

let scrollTimeout: any = null;
let animationFrameId: number | null = null;

const handleManualSeek = (time: number) => {
    isSeeking.value = true;
    setPosition(time);

    // Briefly keep auto-scroll disabled to let the player update
    setTimeout(() => {
        isSeeking.value = false;
        isUserScrolling.value = false;
    }, 20);
};

const gentleScrollTo = (target: number) => {
    const container = scrollContainer.value;
    if (!container || isUserScrolling.value || isSeeking.value) return;

    const start = container.scrollTop;
    const change = target - start;
    const startTime = performance.now();
    const easeOutCubic = (t: number) => (--t) * t * t + 1;

    const animate = (now: number) => {
        const elapsed = now - startTime;
        const progress = Math.min(elapsed / 800, 1); // Increased to 800ms for "gentle" feel
        container.scrollTop = start + change * easeOutCubic(progress);
        if (progress < 1 && !isUserScrolling.value) {
            animationFrameId = requestAnimationFrame(animate);
        }
    };

    if (animationFrameId) cancelAnimationFrame(animationFrameId);
    animationFrameId = requestAnimationFrame(animate);
};

watch(() => currentTime.value, (newTime) => {
    // Prevent auto-scroll if user is interacting or a seek just happened
    if (isUserScrolling.value || isSeeking.value) return;

    const lines = currentMetadata.value?.lyrics || [];
    let index = -1;
    for (let i = 0; i < lines.length; i++) {
        if (lines[i].time > newTime) break;
        index = i;
    }

    if (index !== activeIndex.value && index !== -1) {
        activeIndex.value = index;
        nextTick(() => {
            const container = scrollContainer.value;
            const activeEl = lyricElements.value[index];
            if (container && activeEl) {
                const target = activeEl.offsetTop - (container.clientHeight / 2) + (activeEl.clientHeight / 2);
                gentleScrollTo(target);
            }
        });
    }
});

const startManualScroll = () => {
    isUserScrolling.value = true;
    if (animationFrameId) cancelAnimationFrame(animationFrameId);
    clearTimeout(scrollTimeout);
};

const onScroll = () => {
    if (!isUserScrolling.value || !scrollContainer.value) return;

    const container = scrollContainer.value;
    const centerPoint = container.scrollTop + (container.clientHeight / 2);

    let closestIndex = -1;
    let minDistance = Infinity;

    lyricElements.value.forEach((el, index) => {
        if (!el) return;
        const centerOfEl = el.offsetTop + (el.clientHeight / 2);
        const distance = Math.abs(centerPoint - centerOfEl);
        if (distance < minDistance) {
            minDistance = distance;
            closestIndex = index;
        }
    });

    userHoverIndex.value = closestIndex;

    clearTimeout(scrollTimeout);
    scrollTimeout = setTimeout(() => {
        const selectedLine = currentMetadata.value?.lyrics?.[userHoverIndex.value];
        if (selectedLine && isUserScrolling.value) {
            handleManualSeek(selectedLine.time);
        }
        // userHoverIndex reset is handled after seek to avoid flicker
        userHoverIndex.value = -1;
    }, 800);
};

const formatTime = (seconds: number) => {
    const m = Math.floor(seconds / 60);
    const s = Math.floor(seconds % 60);
    return `${m}:${s.toString().padStart(2, '0')}`;
};

onUnmounted(() => {
    if (animationFrameId) cancelAnimationFrame(animationFrameId);
    clearTimeout(scrollTimeout);
});
</script>

<template>
    <div class="lyrics-view">
        <div class="seek-indicator" :class="{ 'show': isUserScrolling }">
            <div class="seek-time">
                {{ formatTime(currentMetadata.lyrics?.[userHoverIndex]?.time || 0) }}
            </div>
            <div class="seek-line"></div>
            <!-- Added click event: Clicking the icon triggers seek immediately -->
            <div
                class="seek-play"
                @click="handleManualSeek(currentMetadata.lyrics?.[userHoverIndex]?.time || 0)"
            >
                <i class="bi bi-play-circle" style="font-size: 16px;"></i>
            </div>
        </div>

        <div
            class="lyrics-container"
            ref="scrollContainer"
            @scroll="onScroll"
            @wheel="startManualScroll"
            @touchstart="startManualScroll"
        >
            <div class="lyrics-wrapper">
                <p
                    v-for="(line, index) in currentMetadata.lyrics"
                    :key="index"
                    ref="lyricElements"
                    class="lyric-line"
                    :class="{
                        'active': index === activeIndex && !isUserScrolling,
                        'highlight': isUserScrolling && index === userHoverIndex,
                        'dimmed': (index !== activeIndex && !isUserScrolling) || (isUserScrolling && index !== userHoverIndex)
                    }"
                >
                    {{ line.text }}
                </p>
                <div v-if="!currentMetadata.lyrics?.length" class="lyric-line active">
                    Instrumental
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped>
.lyrics-view {
    position: relative;
    width: 100%;
    height: 100%;
    overflow: hidden;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
}

.lyrics-container {
    width: 100%;
    height: 100%;
    overflow-y: auto;
    scrollbar-width: none;
    mask-image: linear-gradient(to bottom, transparent, black 15%, black 85%, transparent);
    -webkit-mask-image: linear-gradient(to bottom, transparent, black 15%, black 85%, transparent);
}

.lyrics-container::-webkit-scrollbar { display: none; }

.lyrics-wrapper {
    text-align: center;
    display: flex;
    flex-direction: column;
    gap: 22px;
    padding: 50% 12%;
}

.lyric-line {
    font-size: 1.1rem;
    font-weight: 700;
    color: white;
    transition: all 0.4s cubic-bezier(0.2, 0, 0.2, 1);
    margin: 0;
    cursor: pointer;
    line-height: 1.3;
}

.lyric-line.active {
    font-size: 1.35rem;
    opacity: 1;
    transform: scale(1.05);
    text-shadow: 0 0 20px var(--text-glow, rgba(255,255,255,0.3));
}

.lyric-line.highlight {
    opacity: 1;
    transform: scale(1.02);
}

.lyric-line.dimmed {
    opacity: 0.3;
}

.seek-indicator {
    position: absolute;
    top: 50%;
    left: 0;
    width: 100%;
    height: 30px;
    transform: translateY(-50%);
    display: flex;
    align-items: center;
    padding: 0 20px;
    box-sizing: border-box;
    pointer-events: none;
    opacity: 0;
    transition: opacity 0.3s ease;
    z-index: 100;
}

.seek-indicator.show {
    opacity: 1;
}

.seek-time {
    font-family: "Geist Mono", monospace;
    font-size: 0.85rem;
    color: rgba(255, 255, 255, 0.6);
    min-width: 45px;
}

.seek-line {
    flex: 1;
    height: 0;
    border-top: 1px dashed rgba(255, 255, 255, 0.25);
    margin: 0 12px;
}

.seek-play {
    color: rgba(255, 255, 255, 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    width: 26px;
    height: 26px;
    transition: all 0.2s;
    background: none;
}

.seek-indicator.show .seek-play {
    pointer-events: auto;
    cursor: pointer;
}
.seek-play:hover {
    background: rgba(255, 255, 255, 0.1);
    color: white;
}
</style>
