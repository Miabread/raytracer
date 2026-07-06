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
    const array = new Uint32Array(e.data);

    if (!imageData || !buffer) {
        const [i, j] = array;
        canvas.width = i;
        canvas.height = j;
        imageData = ctx.createImageData(i, j);
        buffer = new Uint32Array(imageData.data.buffer);
    }

    for (let n = 0; n < array.length; n += 3) {
        const i = array[n];
        const j = array[n + 1];
        const color = array[n + 2];

        buffer[j * canvas.width + i] = color;
    }

    e.data.transfer(0);
};

const render = () => {
    if (imageData) {
        ctx.putImageData(imageData, 0, 0);
    }
    requestAnimationFrame(render);
};
requestAnimationFrame(render);
