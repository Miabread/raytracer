import type { Noise } from './noise';
import { Vec3, type Color3, type Point3 } from './util';

export abstract class Texture {
    public abstract value(u: number, v: number, point: Point3): Color3;
}

export class SolidColor extends Texture {
    constructor(private albedo: Color3) {
        super();
    }

    public value(u: number, v: number, point: Point3): Color3 {
        return this.albedo;
    }
}

export class Checker extends Texture {
    private invertedScale: number;
    constructor(
        scale: number,
        private even: Texture,
        private odd: Texture,
    ) {
        super();
        this.invertedScale = 1 / scale;
    }

    public value(u: number, v: number, point: Point3): Color3 {
        return point //
            .map((n) => Math.floor(this.invertedScale * n))
            .fold((x, y, z) => (x + y + z) % 2 === 0)
            ? this.even.value(u, v, point)
            : this.odd.value(u, v, point);
    }
}

export class NoiseTexture extends Texture {
    constructor(
        private noise: Noise,
        private scale: number,
    ) {
        super();
    }

    public value(u: number, v: number, point: Point3): Color3 {
        const noise = this.noise.noise(point.times(this.scale));
        return Vec3.one.times(0.5 * (1 + noise));
    }
}

export class TurbulenceTexture extends Texture {
    constructor(
        private noise: Noise,
        private scale: number,
    ) {
        super();
    }

    public value(u: number, v: number, point: Point3): Color3 {
        return Vec3.of(0.5).times(1 + Math.sin(this.scale * point.z + 10 * this.noise.turbulence(point, 7)));
    }
}
