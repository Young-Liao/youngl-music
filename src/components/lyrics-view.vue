<script setup lang="ts">
import { ref, watch, nextTick, onUnmounted } from 'vue';
import { currentMetadata, currentTime } from '../scripts/globals';

const scrollContainer = ref<HTMLElement | null>(null);
const lyricElements = ref<HTMLElement[]>([]);

const activeIndex = ref(-1);         // 当前播放行
const userHoverIndex = ref(-1);      // 滚轮指向行
const isUserScrolling = ref(false);  // 是否正在手动滚动

let scrollTimeout: any = null;
let animationFrameId: number | null = null;

// 平滑滚动逻辑 (保持 600ms，手感最接近 iOS)
const gentleScrollTo = (target: number) => {
    const container = scrollContainer.value;
    if (!container) return;

    const start = container.scrollTop;
    const change = target - start;
    const startTime = performance.now();
    const easeOutCubic = (t: number) => (--t) * t * t + 1;

    const animate = (now: number) => {
        const elapsed = now - startTime;
        const progress = Math.min(elapsed / 600, 1);
        container.scrollTop = start + change * easeOutCubic(progress);
        if (progress < 1) animationFrameId = requestAnimationFrame(animate);
    };

    if (animationFrameId) cancelAnimationFrame(animationFrameId);
    animationFrameId = requestAnimationFrame(animate);
};

// 监听播放进度
watch(() => currentTime.value, (newTime) => {
    if (isUserScrolling.value) return; // 手动滚动时不自动对齐

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

// 手动滚动处理
const startManualScroll = () => {
    isUserScrolling.value = true;
    clearTimeout(scrollTimeout);
};

const onScroll = () => {
    if (!isUserScrolling.value || !scrollContainer.value) return;

    const container = scrollContainer.value;
    const centerPoint = container.scrollTop + (container.clientHeight / 2);

    let closestIndex = -1;
    let minDistance = Infinity;

    lyricElements.value.forEach((el, index) => {
        const centerOfEl = el.offsetTop + (el.clientHeight / 2);
        const distance = Math.abs(centerPoint - centerOfEl);
        if (distance < minDistance) {
            minDistance = distance;
            closestIndex = index;
        }
    });

    userHoverIndex.value = closestIndex;

    // 停止操作 1.5秒后恢复自动对齐并隐藏时间轴
    clearTimeout(scrollTimeout);
    scrollTimeout = setTimeout(() => {
        isUserScrolling.value = false;
        userHoverIndex.value = -1;
    }, 1500);
};

const formatTime = (seconds: number) => {
    const m = Math.floor(seconds / 60);
    const s = Math.floor(seconds % 60);
    return `${m}:${s.toString().padStart(2, '0')}`;
};

onUnmounted(() => {
    cancelAnimationFrame(animationFrameId!);
    clearTimeout(scrollTimeout);
});
</script>

<template>
    <div class="lyrics-view">
        <!-- 手动滚动指示器：仅在手动时显示 -->
        <div class="seek-indicator" :class="{ 'show': isUserScrolling }">
            <div class="line"></div>
            <div class="time-box">
                {{ formatTime(currentMetadata.lyrics?.[userHoverIndex]?.time || 0) }}
            </div>
            <div class="line"></div>
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
                        'dimmed': (index !== activeIndex && !isUserScrolling) || (isUserScrolling && index !== userHoverIndex),
                        'highlight': isUserScrolling && index === userHoverIndex
                    }"
                >
                    {{ line.text }}
                </p>
                <p v-if="!currentMetadata.lyrics?.length" class="lyric-line active">
                    Waiting for lyrics...
                </p>
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
    /* 你的背景色或者是透明 */
}

.lyrics-container {
    width: 100%;
    height: 100%;
    overflow-y: auto;
    scrollbar-width: none;
    -ms-overflow-style: none;
    mask-image: linear-gradient(to bottom, transparent, black 15%, black 85%, transparent);
    -webkit-mask-image: linear-gradient(to bottom, transparent, black 15%, black 85%, transparent);
}

.lyrics-container::-webkit-scrollbar { display: none; }

.lyrics-wrapper {
    text-align: center;
    display: flex;
    flex-direction: column;
    gap: 16px; /* 恢复你的 16px 间距 */
    padding: 50% 20px; /* 关键：确保上下有足够留白实现居中 */
}

.lyric-line {
    font-family: var(--font-main);
    font-size: 1rem; /* 非活跃行大小 */
    font-weight: 600;
    color: rgba(255, 255, 255, 0.3); /* 你的 dimmed 颜色 */
    transition: all 0.35s cubic-bezier(0.4, 0, 0.2, 1);
    margin: 0;
    cursor: pointer;
}

/* 播放中的状态 */
.lyric-line.active {
    font-size: 1.2rem;
    color: #ffffff;
    transform: scale(1.05);
    text-shadow: 0 0 15px var(--text-glow);
}

/* 手动滚动时，中间那行的状态 */
.lyric-line.highlight {
    color: rgba(255, 255, 255, 0.9);
    transform: scale(1.02);
}

.lyric-line.dimmed {
    /* 统一非焦点行的透明度 */
    opacity: 0.3;
}

/* --- 手动滚动时间指示器 --- */
.seek-indicator {
    position: absolute;
    top: 50%;
    left: 0;
    width: 100%;
    transform: translateY(-50%);
    display: flex;
    align-items: center;
    justify-content: center;
    pointer-events: none;
    opacity: 0;
    transition: opacity 0.2s ease;
    z-index: 10;
}

.seek-indicator.show {
    opacity: 1;
}

.seek-indicator .line {
    flex: 1;
    height: 1px;
    background: linear-gradient(90deg,
    transparent,
    rgba(255, 255, 255, 0.2) 20%,
    rgba(255, 255, 255, 0.2) 80%,
    transparent
    );
    border-top: 1px dashed rgba(255, 255, 255, 0.2);
}

.seek-indicator .time-box {
    margin: 0 15px;
    font-family: monospace;
    font-size: 0.85rem;
    color: rgba(255, 255, 255, 0.5);
    background: rgba(0, 0, 0, 0.3);
    padding: 2px 8px;
    border-radius: 4px;
    backdrop-filter: blur(4px);
}
</style>
