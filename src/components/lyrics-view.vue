<template>
    <div class="lyrics-container">
        <div class="lyrics-scroll">
            <p class="lyric-line dimmed">Pure imagination...</p>
            <p class="lyric-line active">Waiting for the lyrics...</p>
            <p class="lyric-line dimmed">To be implemented.</p>
        </div>
    </div>
</template>

<style scoped>
.lyrics-container {
    flex: 1;
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    mask-image: linear-gradient(to bottom, transparent, black 15%, black 85%, transparent);
    -webkit-mask-image: linear-gradient(to bottom, transparent, black 15%, black 85%, transparent);
    overflow: hidden;
}

.lyrics-scroll {
    text-align: center;
    display: flex;
    flex-direction: column;
    gap: 16px;
}

.lyric-line {
    font-family: var(--font-main);
    font-size: 1.2rem;
    font-weight: 600;
    transition: all 0.3s ease;
}

.lyric-line.active {
    color: #ffffff;
    transform: scale(1.05);
    text-shadow: 0 0 15px var(--text-glow);
}

.lyric-line.dimmed {
    color: rgba(255, 255, 255, 0.3);
    font-size: 1rem;
}
</style>
