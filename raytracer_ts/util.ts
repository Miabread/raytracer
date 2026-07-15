import type { Ray } from './hittable';

export const degreesToRadians = (x: number) => x * (Math.PI / 180);

export class Vec3 {
    static get zero() {
        return Vec3.of(0);
    }
    static get one() {
        return Vec3.of(1);
    }
    public static of(n: number) {
        return new Vec3(n, n, n);
    }

    constructor(
        public x: number,
        public y: number,
        public z: number,
    ) {}

    static random(interval: Interval): Vec3 {
        return new Vec3(interval.random(), interval.random(), interval.random());
    }
    static randomUnitVector(): Vec3 {
        while (true) {
            const point = Vec3.random(new Interval(-1, 1));
            if (1e-600 < point.lengthSquared && point.lengthSquared <= 1) {
                return point.div(Math.sqrt(point.lengthSquared));
            }
        }
    }
    static randomOnHemisphere(normal: Vec3): Vec3 {
        const onUnitSphere = this.randomUnitVector();
        return onUnitSphere.dot(normal) > 0.0 ? onUnitSphere : onUnitSphere.neg;
    }
    static randomInUnitDisk() {
        const interval = new Interval(-1, 1);
        while (true) {
            const point = new Vec3(interval.random(), interval.random(), 0);
            if (point.lengthSquared < 1) {
                return point;
            }
        }
    }

    get r() {
        return this.x;
    }
    get g() {
        return this.y;
    }
    get b() {
        return this.z;
    }
    public index(n: number) {
        if (n === 1) return this.y;
        if (n === 2) return this.z;
        return this.x;
    }

    get lengthSquared() {
        return this.x ** 2 + this.y ** 2 + this.z ** 2;
    }
    get length() {
        return Math.sqrt(this.lengthSquared);
    }
    get unitVector() {
        return this.div(this.length);
    }
    get nearZero() {
        const eps = 1e-8;
        return Math.abs(this.x) < eps && Math.abs(this.y) < eps && Math.abs(this.z) < eps;
    }

    get neg(): Vec3 {
        return new Vec3(-this.x, -this.y, -this.z);
    }
    public plus(input: Vec3 | number): Vec3 {
        if (typeof input === 'number') {
            return new Vec3(this.x + input, this.y + input, this.z + input);
        }
        return new Vec3(this.x + input.x, this.y + input.y, this.z + input.z);
    }
    public minus(input: Vec3 | number): Vec3 {
        if (typeof input === 'number') {
            return new Vec3(this.x - input, this.y - input, this.z - input);
        }
        return new Vec3(this.x - input.x, this.y - input.y, this.z - input.z);
    }
    public times(input: Vec3 | number): Vec3 {
        if (typeof input === 'number') {
            return new Vec3(this.x * input, this.y * input, this.z * input);
        }
        return new Vec3(this.x * input.x, this.y * input.y, this.z * input.z);
    }
    public div(input: Vec3 | number): Vec3 {
        if (typeof input === 'number') {
            return new Vec3(this.x / input, this.y / input, this.z / input);
        }
        return new Vec3(this.x / input.x, this.y / input.y, this.z / input.z);
    }
    public min(input: Vec3 | number): Vec3 {
        if (typeof input === 'number') {
            return new Vec3(Math.min(this.x, input), Math.min(this.y, input), Math.min(this.z, input));
        }
        return new Vec3(Math.min(this.x, input.x), Math.min(this.y, input.y), Math.min(this.z, input.z));
    }
    public max(input: Vec3 | number): Vec3 {
        if (typeof input === 'number') {
            return new Vec3(Math.max(this.x, input), Math.max(this.y, input), Math.max(this.z, input));
        }
        return new Vec3(Math.max(this.x, input.x), Math.max(this.y, input.y), Math.max(this.z, input.z));
    }

    public map(func: (n: number) => number): Vec3 {
        return new Vec3(func(this.x), func(this.y), func(this.z));
    }
    public fold<R>(func: (x: number, y: number, z: number) => R): R {
        return func(this.x, this.y, this.z);
    }

    public dot(input: Vec3): number {
        return this.x * input.x + this.y * input.y + this.z * input.z;
    }
    public cross(input: Vec3): Vec3 {
        return new Vec3(
            this.y * input.z - this.z * input.y,
            this.z * input.x - this.x * input.z,
            this.x * input.y - this.y * input.x,
        );
    }
    public reflect(input: Vec3): Vec3 {
        return this.minus(input.times(2 * this.dot(input)));
    }
    public refract(input: Vec3, etaIOverEtaT: number) {
        const cosTheta = Math.min(this.neg.dot(input), 1.0);
        const resultPerpendicular = this.plus(input.times(cosTheta)).times(etaIOverEtaT);
        const resultParallel = input.times(-Math.sqrt(Math.abs(1.0 - resultPerpendicular.lengthSquared)));
        return resultPerpendicular.plus(resultParallel);
    }
}

export type Point3 = Vec3;
export type Color3 = Vec3;

export class Interval {
    static get empty() {
        return new Interval(+Infinity, -Infinity);
    }
    static get full() {
        return new Interval(-Infinity, +Infinity);
    }
    static get unit() {
        return new Interval(0, 1);
    }

    constructor(
        public min: number,
        public max: number,
    ) {}

    get size() {
        return this.max - this.min;
    }

    public expand(delta: number) {
        const padding = delta / 2;
        return new Interval(this.min - padding, this.max + padding);
    }

    public contains(x: number) {
        return this.min <= x && x <= this.max;
    }

    public surrounds(x: number) {
        return this.min < x && x < this.max;
    }

    public clamp(x: number) {
        if (x < this.min) return this.min;
        if (x > this.max) return this.max;
        return x;
    }

    public random() {
        return Math.random() * this.size + this.min;
    }
    public randomInteger() {
        return Math.trunc(new Interval(this.min, this.max + 1).random());
    }

    public join(other: Interval) {
        return new Interval(this.min <= other.min ? this.min : other.min, this.max >= other.max ? this.max : other.max);
    }
}

export class BoundingBox {
    public static get empty() {
        return new BoundingBox(Interval.empty, Interval.empty, Interval.empty);
    }

    constructor(
        public x: Interval,
        public y: Interval,
        public z: Interval,
    ) {
        const delta = 0.0001;
        if (this.x.size < delta) this.x = this.x.expand(delta);
        if (this.y.size < delta) this.y = this.y.expand(delta);
        if (this.z.size < delta) this.z = this.z.expand(delta);
    }

    public static corners(a: Point3, b: Point3) {
        return new BoundingBox(
            a.x <= b.x ? new Interval(a.x, b.x) : new Interval(b.x, a.x),
            a.y <= b.y ? new Interval(a.y, b.y) : new Interval(b.y, a.y),
            a.z <= b.z ? new Interval(a.z, b.z) : new Interval(b.z, a.z),
        );
    }

    public join(other: BoundingBox) {
        return new BoundingBox(this.x.join(other.x), this.y.join(other.y), this.z.join(other.z));
    }

    public index(n: number) {
        if (n === 1) return this.y;
        if (n === 2) return this.z;
        return this.x;
    }

    public longestAxis() {
        if (this.x.size > this.y.size) {
            return this.x.size > this.z.size ? 0 : 2;
        }
        return this.y.size > this.z.size ? 1 : 2;
    }

    public hit(ray: Ray, rayT: Interval) {
        let tMin = rayT.min;
        let tMax = rayT.max;

        for (let i = 0; i < 3; i++) {
            const axis = this.index(i);
            const inverted = 1.0 / ray.direction.index(i);

            let t0 = (axis.min - ray.origin.index(i)) * inverted;
            let t1 = (axis.max - ray.origin.index(i)) * inverted;

            if (inverted < 0) {
                [t0, t1] = [t1, t0];
            }

            if (t0 > tMin) tMin = t0;
            if (t1 < tMax) tMax = t1;

            if (tMax <= tMin) {
                return false;
            }
        }

        return true;
    }
}
