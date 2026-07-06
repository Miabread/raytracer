/** @type {HTMLCanvasElement} */
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

const worker = new Worker(new URL('./worker.js', import.meta.url), { type: 'module' });

worker.postMessage({ aspectRatio: window.innerWidth / window.innerHeight });

console.log('worker posted');

/** @type {ImageData} */
let imageData = null;
/** @type {Uint32Array} */
let buffer = null;

worker.onmessage = (/** @type {MessageEvent<ArrayBuffer>} */ e) => {
    const [i, j, color] = new Uint32Array(e.data);
    e.data.transfer(0);

    if (!imageData || !buffer) {
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
