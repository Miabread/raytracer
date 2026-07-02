import { draw } from './pkg';

const canvas = document.getElementById('canvas');

const resizeCanvas = () => {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
};

resizeCanvas();
window.addEventListener('resize', resizeCanvas);

draw();
