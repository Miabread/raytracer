import { Interval, Vec3, type Point3 } from './util';

export abstract class Noise {
    public abstract noise(point: Point3): number;

    public turbulence(initialPoint: Point3, depth: number) {
        let accumulator = 0.0;
        let weight = 1.0;
        let point = initialPoint;

        for (let i = 0; i < depth; i++) {
            accumulator += weight * this.noise(point);
            weight *= 0.5;
            point = point.times(2);
        }

        return Math.abs(accumulator);
    }
}

export class Perlin extends Noise {
    private randomVec: Vec3[];
    private permX: number[];
    private permY: number[];
    private permZ: number[];

    constructor(private pointCount = 256) {
        super();
        this.randomVec = new Array(this.pointCount);
        this.permX = new Array(this.pointCount);
        this.permY = new Array(this.pointCount);
        this.permZ = new Array(this.pointCount);

        const interval = new Interval(-1, 1);
        for (let i = 0; i < this.pointCount; i++) {
            this.randomVec[i] = Vec3.random(interval).unitVector;
        }

        this.perlinGeneratePerm(this.permX);
        this.perlinGeneratePerm(this.permY);
        this.perlinGeneratePerm(this.permZ);
    }

    private perlinGeneratePerm(p: number[]) {
        for (let i = 0; i < this.pointCount; i++) {
            p[i] = i;
        }

        this.permute(p, this.pointCount);
    }

    private permute(p: number[], n: number) {
        for (let i = n - 1; i > 0; i--) {
            const target = new Interval(0, i).randomInteger();

            const temp = p[i];
            p[i] = p[target];
            p[target] = temp;
        }
    }

    public noise(point: Point3): number {
        const u = point.x - Math.floor(point.x);
        const v = point.y - Math.floor(point.y);
        const w = point.z - Math.floor(point.z);

        const i = Math.floor(point.x);
        const j = Math.floor(point.y);
        const k = Math.floor(point.z);

        const c = [
            [new Array(3), new Array(3), new Array(3)],
            [new Array(3), new Array(3), new Array(3)],
            [new Array(3), new Array(3), new Array(3)],
        ];

        for (let di = 0; di < 2; di++) {
            for (let dj = 0; dj < 2; dj++) {
                for (let dk = 0; dk < 2; dk++) {
                    c[di][dj][dk] =
                        this.randomVec[
                            this.permX[(i + di) & 255] ^ this.permY[(j + dj) & 255] ^ this.permZ[(k + dk) & 255]
                        ];
                }
            }
        }

        return this.interpolate(c, u, v, w);
    }

    private interpolate(c: Vec3[][][], u: number, v: number, w: number) {
        const uu = u * u * (3 - 2 * u);
        const vv = v * v * (3 - 2 * v);
        const ww = w * w * (3 - 2 * w);
        let accumulator = 0.0;

        for (let i = 0; i < 2; i++) {
            for (let j = 0; j < 2; j++) {
                for (let k = 0; k < 2; k++) {
                    const weight = new Vec3(u - i, v - j, w - k);
                    accumulator +=
                        (i * uu + (1 - i) * (1 - uu)) *
                        (j * vv + (1 - j) * (1 - vv)) *
                        (k * ww + (1 - k) * (1 - ww)) *
                        c[i][j][k].dot(weight);
                }
            }
        }

        return accumulator;
    }
}
