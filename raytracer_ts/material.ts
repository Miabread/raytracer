import { Ray, type HitResult } from './hittable';
import { type Texture } from './texture';
import { Interval, Vec3, type Color3, type Point3 } from './util';

export class MaterialResult {
    constructor(
        public attenuation: Color3,
        public scattered: Ray,
    ) {}
}

export abstract class Material {
    public abstract scatter(ray: Ray, hit: HitResult): MaterialResult | null;

    public emitted(u: number, v: number, point: Point3) {
        return Vec3.zero;
    }
}

export class Lambert extends Material {
    constructor(private texture: Texture) {
        super();
    }

    public override scatter(ray: Ray, hit: HitResult): MaterialResult | null {
        let scatterDirection = hit.normal.plus(Vec3.randomUnitVector());

        if (scatterDirection.nearZero) {
            scatterDirection = hit.normal;
        }

        const scattered = new Ray(hit.point, scatterDirection, ray.time);
        const attenuation = this.texture.value(hit.u, hit.v, hit.point);
        return new MaterialResult(attenuation, scattered);
    }
}

export class Metal extends Material {
    constructor(
        private albedo: Color3,
        private fuzz: number,
    ) {
        super();
    }

    public override scatter(ray: Ray, hit: HitResult): MaterialResult | null {
        const reflected = ray.direction.reflect(hit.normal);
        const fuzzed = reflected.unitVector.plus(Vec3.randomUnitVector().times(this.fuzz));
        const scattered = new Ray(hit.point, fuzzed, ray.time);

        if (scattered.direction.dot(hit.normal) > 0) {
            return new MaterialResult(this.albedo, scattered);
        }

        return null;
    }
}

export class Dielectric extends Material {
    constructor(private refractionIndex: number) {
        super();
    }

    public scatter(ray: Ray, hit: HitResult): MaterialResult | null {
        const refractionIndex = hit.frontFace ? 1.0 / this.refractionIndex : this.refractionIndex;

        const unitDirection = ray.direction.unitVector;
        const cosTheta = Math.min(unitDirection.neg.dot(hit.normal), 1.0);
        const sinTheta = Math.sqrt(1.0 - cosTheta ** 2);

        const cannotRefract = refractionIndex * sinTheta > 1.0;

        const direction =
            cannotRefract || this.reflectance(cosTheta) > Interval.unit.random()
                ? unitDirection.reflect(hit.normal)
                : unitDirection.refract(hit.normal, refractionIndex);

        return new MaterialResult(Vec3.one, new Ray(hit.point, direction, ray.time));
    }

    private reflectance(cosine: number) {
        const r0 = ((1 - this.refractionIndex) / (1 + this.refractionIndex)) ** 2;
        return r0 + (1 - r0) * (1 - cosine) ** 5;
    }
}

export class DiffuseLight extends Material {
    constructor(private texture: Texture) {
        super();
    }

    public scatter(ray: Ray, hit: HitResult): MaterialResult | null {
        return null;
    }

    public emitted(u: number, v: number, point: Point3): Vec3 {
        return this.texture.value(u, v, point);
    }
}
