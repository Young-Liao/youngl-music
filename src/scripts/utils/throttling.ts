export const throttle = (fn: Function, wait: number) => {
    let lastCall = 0;
    return (...args: any[]) => {
        const now = Date.now();
        if (now - lastCall >= wait) {
            lastCall = now;
            return fn(...args);
        }
    };
}

export const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
