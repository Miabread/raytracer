const canvas = document.getElementById('canvas');

const offscreen = canvas.transferControlToOffscreen();

const worker = new Worker(new URL('./worker.js', import.meta.url), { type: 'module' });

worker.postMessage({ canvas: offscreen, aspectRatio: window.innerWidth / window.innerHeight }, [offscreen]);

console.log('worker posted');
