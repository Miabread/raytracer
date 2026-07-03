/** @type {HTMLCanvasElement} */
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

const worker = new Worker(new URL('./worker.js', import.meta.url), { type: 'module' });

worker.postMessage({ aspectRatio: window.innerWidth / window.innerHeight });

console.log('worker posted');

/** @type {ImageData} */
let imageData = null;
let buf32 = null;

worker.onmessage = (/** @type {MessageEvent} */ e) => {
    const [i, j, color] = e.data;

    if (!imageData) {
        canvas.width = i;
        canvas.height = j;
        imageData = ctx.createImageData(i, j);
        buffer = new Uint32Array(imageData.data.buffer);
    }

    buffer[j * canvas.width + i] = color;
};

const render = () => {
    if (imageData) {
        ctx.putImageData(imageData, 0, 0);
    }
    requestAnimationFrame(render);
};
requestAnimationFrame(render);
