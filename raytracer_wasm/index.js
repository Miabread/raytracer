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

let scanline = 0;
let scanlineColor = 'blue';

const handlePixelBatch = (/** @type {MessageEvent<ArrayBuffer>} */ e) => {
    const array = new Uint32Array(e.data);

    for (let n = 0; n < array.length; n += 3) {
        const i = array[n];
        const j = array[n + 1];
        const color = array[n + 2];

        buffer[j * canvas.width + i] = color;
    }

    // Get the height of the last received pixel
    scanline = array[array.length - 2];

    e.data.transfer(0);
};

// By protocol, first pixel sent determines canvas width and height
worker.onmessage = (/** @type {MessageEvent<ArrayBuffer>} */ e) => {
    worker.onmessage = handlePixelBatch;

    const [i, j] = new Uint32Array(e.data);
    canvas.width = i;
    canvas.height = j;
    imageData = ctx.createImageData(i, j);
    buffer = new Uint32Array(imageData.data.buffer);

    handlePixelBatch(e);
};

const render = () => {
    if (imageData) {
        ctx.putImageData(imageData, 0, 0);

        if (scanlineColor) {
            ctx.fillStyle = scanlineColor;
            ctx.fillRect(0, scanline, canvas.width, canvas.height / 200);
        }
    }
    requestAnimationFrame(render);
};
requestAnimationFrame(render);

const scanlineColors = {
    r: 'red',
    g: 'green',
    b: 'blue',
    w: 'white',
    a: 'black',
    ' ': null,
};

document.addEventListener('keydown', (event) => {
    if (Object.hasOwn(scanlineColors, event.key)) {
        scanlineColor = scanlineColors[event.key];
    }
});
