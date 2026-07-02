import init, { draw } from './pkg';

self.onmessage = async (event) => {
    console.log('worker received');

    await init();

    draw(event.data.canvas, event.data.aspectRatio);
};
