/** @type {HTMLCanvasElement} */
const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

const worker = new Worker(new URL('./worker.js', import.meta.url), { type: 'module' });

const aspectRatio = window.innerWidth / window.innerHeight;
const imageWidth = 400;
const imageHeight = Math.floor(imageWidth / aspectRatio);

canvas.width = imageWidth;
canvas.height = imageHeight;

const imageData = ctx.createImageData(imageWidth, imageHeight);
const buffer = new Uint32Array(imageData.data.buffer);

let scanline = 0;
let scanlineColor = 'blue';

worker.onmessage = (/** @type {MessageEvent<ArrayBuffer>} */ e) => {
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

worker.postMessage({ imageWidth, imageHeight });

const render = () => {
    ctx.putImageData(imageData, 0, 0);

    if (scanlineColor) {
        ctx.fillStyle = scanlineColor;
        ctx.fillRect(0, scanline, canvas.width, canvas.height / 200);
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
