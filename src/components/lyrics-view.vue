<script setup lang="ts">
import {ref, watch, nextTick, onUnmounted, onMounted} from 'vue';
import {currentMetadata, currentTime} from '../scripts/globals';
import {setPosition} from "../scripts/playback/progress-controller.ts";

// 定义信号：点击背景时通知父组件
const emit = defineEmits(['toggle-view']);

const scrollContainer = ref<HTMLElement | null>(null);
const lyricElements = ref<HTMLElement[]>([]);

// 状态变量
const activeIndex = ref(-1);       // 当前播放进度对应的歌词索引
const userHoverIndex = ref(-1);    // 用户划动时，屏幕中心对准的歌词索引
const isUserScrolling = ref(false); // 标记：用户是否正在交互
const isResizing = ref(false);
let resizeTimer: any = null;

// 内部控制变量
let scrollTimeout: any = null;     // 用于检测滚动停止的计时器
let autoScrollFrame: number | null = null; // 动画帧句柄
let isAutoScrolling = false;       // 锁：标记当前滚动是否由代码触发（防止onScroll误判）

const setAutoScrollLock = (value: boolean) => {
    isAutoScrolling = value;
    if (!value) {
        // 如果是释放锁，通常建议顺便清理掉可能的计时器
        isUserScrolling.value = false;
    }
};

// --- 辅助：计算屏幕中心对应的歌词行 ---
const calculateCenterLine = () => {
    const container = scrollContainer.value;
    if (!container) return;

    const centerPoint = container.scrollTop + (container.clientHeight / 2);
    let closestIndex = -1;
    let minDistance = Infinity;

    lyricElements.value.forEach((el, index) => {
        if (!el) return;
        // 计算每一行的中心点
        const centerOfEl = el.offsetTop + (el.clientHeight / 2);
        const distance = Math.abs(centerPoint - centerOfEl);
        if (distance < minDistance) {
            minDistance = distance;
            closestIndex = index;
        }
    });

    userHoverIndex.value = closestIndex;
};

const formatTime = (seconds: number) => {
    const m = Math.floor(seconds / 60);
    const s = Math.floor(seconds % 60);
    return `${m}:${s.toString().padStart(2, '0')}`;
};

// --- 核心：处理滚动事件 ---
const onScroll = () => {
    // 1. 如果当前是由代码触发的自动滚动，直接忽略，不视为用户操作
    if (isAutoScrolling || isResizing.value) return;

    // 2. 标记为用户正在滚动
    isUserScrolling.value = true;

    // 3. 实时计算当前中心行（用于显示时间指示器）
    calculateCenterLine();

    // 4. 重置超时计时器（“看门狗”机制）
    clearTimeout(scrollTimeout);

    // 5. 设置停止检测：如果 0.8秒 内没有新的 scroll 事件，视为操作结束
    scrollTimeout = setTimeout(async () => {
        const selectedTime = currentMetadata.value?.lyrics?.[userHoverIndex.value]?.time;
        await setPosition(selectedTime as number);

        // 操作结束后，平滑归位到当前播放行
        scrollToActiveLine();
        isUserScrolling.value = false;
        userHoverIndex.value = -1;
    }, 800);
};

// --- 用户开始交互（触摸/滚轮） ---
// --- 核心：平滑滚动引擎 ---
const gentleScrollTo = (target: number) => {
    const container = scrollContainer.value;
    if (!container) return;

    const start = container.scrollTop;
    const change = target - start;
    const startTime = performance.now();
    const duration = 600; // 动画持续时间 ms

    // 标记开始自动滚动
    isAutoScrolling = true;

    const animate = (now: number) => {
        // 如果中途用户介入了，立即停止动画
        if (isUserScrolling.value) {
            isAutoScrolling = false;
            return;
        }

        const elapsed = now - startTime;
        const progress = Math.min(elapsed / duration, 1);

        // 缓动函数 (Ease Out Cubic)
        const ease = (t: number) => (--t) * t * t + 1;

        container.scrollTop = start + change * ease(progress);

        if (progress < 1) {
            autoScrollFrame = requestAnimationFrame(animate);
        } else {
            // 动画结束，释放锁
            // 稍微延迟一点释放，以免惯性导致 onScroll 误判
            setTimeout(() => {
                isAutoScrolling = false;
            }, 50);
        }
    };

    if (autoScrollFrame) cancelAnimationFrame(autoScrollFrame);
    autoScrollFrame = requestAnimationFrame(animate);
};

// --- 动作：滚动到当前激活行 ---
const scrollToActiveLine = () => {
    if (isUserScrolling.value) return; // 如果用户在看别处，不要打扰

    const container = scrollContainer.value;
    const activeEl = lyricElements.value[activeIndex.value];

    if (container && activeEl) {
        // 计算目标位置：让元素中心 对齐 容器中心
        const target = activeEl.offsetTop - (container.clientHeight / 2) + (activeEl.clientHeight / 2);
        gentleScrollTo(target);
    }
};

// --- 监听播放进度 ---
watch(() => currentTime.value, (newTime) => {
    // 如果用户正在操作，仅更新 activeIndex 高亮，不滚动
    if (isUserScrolling.value) return;

    const lines = currentMetadata.value?.lyrics || [];
    let index = -1;
    // 找到当前时间对应的最后一行歌词
    for (let i = 0; i < lines.length; i++) {
        if (lines[i].time > newTime) break;
        index = i;
    }

    // 只有行数变化时才触发滚动
    if (index !== activeIndex.value && index !== -1) {
        activeIndex.value = index;
        nextTick(() => {
            scrollToActiveLine();
        });
    }
});

// --- 清理工作 ---
onUnmounted(() => {
    if (autoScrollFrame) cancelAnimationFrame(autoScrollFrame);
    clearTimeout(scrollTimeout);
});

// --- 核心：点击 vs 滚动的判定变量 ---
let startX = 0;
let startY = 0;
let startTime = 0;

// 用户开始交互（记录起点）
const onUserInteractStart = (e: TouchEvent | MouseEvent) => {
    // 记录开始位置和时间
    startX = 'touches' in e ? e.touches[0].clientX : e.clientX;
    startY = 'touches' in e ? e.touches[0].clientY : e.clientY;
    startTime = Date.now();

    isUserScrolling.value = true;
    if (autoScrollFrame) cancelAnimationFrame(autoScrollFrame);
    isAutoScrolling = false;
};

// 用户结束交互（判定是否触发 toggle-view）
const handleInteractionEnd = (e: TouchEvent | MouseEvent) => {
    const endX = 'changedTouches' in e ? e.changedTouches[0].clientX : (e instanceof MouseEvent ? e.clientX : 0);
    const endY = 'changedTouches' in e ? e.changedTouches[0].clientY : (e instanceof MouseEvent ? e.clientY : 0);
    const duration = Date.now() - startTime;

    // 计算移动距离
    const distance = Math.sqrt(Math.pow(endX - startX, 2) + Math.pow(endY - startY, 2));

    // 判定逻辑：如果移动距离很小且按下时间很短，判定为“点按”，发射切换信号
    if (distance < 5 && duration < 250) {
        emit('toggle-view');
        isUserScrolling.value = false;
        userHoverIndex.value = -1;
    }
};

defineExpose({setAutoScrollLock, scrollToActiveLine});

// LyricsView.vue

const handleResize = () => {
    // 1. 开启 Resize 锁
    isResizing.value = true;

    // 2. 如果 Resize 期间正在进行自动滚动动画，立即停止
    if (autoScrollFrame) cancelAnimationFrame(autoScrollFrame);
    isAutoScrolling = false;

    // 3. 防抖处理：窗口停止变动 200ms 后才解锁
    clearTimeout(resizeTimer);
    resizeTimer = setTimeout(() => {
        isResizing.value = false;
        // 4. 解锁后，重新将当前歌词对齐到中心
        scrollToActiveLine();
    }, 200);
};

// 在 onMounted 中绑定，onUnmounted 中销毁
onMounted(() => {
    window.addEventListener('resize', handleResize);
});

onUnmounted(() => {
    window.removeEventListener('resize', handleResize);
});

</script>

<template>
    <div class="lyrics-view">
        <div class="seek-indicator" :class="{ 'show': isUserScrolling }">
            <div class="seek-time">
                {{ formatTime(currentMetadata.lyrics?.[userHoverIndex]?.time || 0) }}
            </div>
            <div class="seek-line"></div>
            <div class="seek-play">
                <i class="bi bi-play-circle-fill"></i>
            </div>
        </div>

        <div class="lyrics-container"
             ref="scrollContainer"
             @scroll="onScroll"
             @wheel="onUserInteractStart"
             @mousedown="onUserInteractStart"
             @mouseup="handleInteractionEnd"
             @touchstart="onUserInteractStart"
             @touchend="handleInteractionEnd"
             @click.stop>

            <div class="lyrics-wrapper">
                <div v-if="!currentMetadata.lyrics?.length" class="lyric-line active">
                    Instrumental / No Lyrics
                </div>

                <p v-for="(line, index) in currentMetadata.lyrics"
                   :key="index"
                   ref="lyricElements"
                   class="lyric-line"
                   :class="{
                       'active': index === activeIndex && !isUserScrolling,
                       'highlight': isUserScrolling && index === userHoverIndex,
                       'dimmed': (index !== activeIndex && !isUserScrolling) || (isUserScrolling && index !== userHoverIndex)
                   }">
                    {{ line.text }}
                </p>
                <div class="lyrics-spacer"></div>
            </div>
        </div>
    </div>
</template>

<style scoped src="../styles/components/lyrics-view.css" />
