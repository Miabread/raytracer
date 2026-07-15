import type { Material } from './material';
import { type Point3, BoundingBox, Interval, Vec3 } from './util';

export class Ray {
    constructor(
        public origin: Point3,
        public direction: Vec3,
        public time: number,
    ) {}

    public at(t: number): Point3 {
        return this.origin.plus(this.direction.times(t));
    }
}

interface HitResultOptions {
    material: Material;
    t: number;
    point: Point3;
    ray: Ray;
    outwardNormal: Vec3;
    u: number;
    v: number;
}

export class HitResult {
    public material: Material;
    public t: number;
    public point: Point3;
    public u: number;
    public v: number;

    public normal: Vec3;
    public frontFace: boolean;

    constructor(options: HitResultOptions) {
        this.material = options.material;
        this.t = options.t;
        this.point = options.point;
        this.u = options.u;
        this.v = options.v;

        this.frontFace = options.ray.direction.dot(options.outwardNormal) < 0;
        this.normal = this.frontFace ? options.outwardNormal : options.outwardNormal.neg;
    }
}

export abstract class Hittable {
    public boundingBox = BoundingBox.empty;

    public abstract hit(ray: Ray, rayT: Interval): HitResult | null;
}

export class Sphere extends Hittable {
    public static stationary(staticCenter: Point3, radius: number, material: Material) {
        const center = new Ray(staticCenter, Vec3.zero, 0);

        const radiusVec = Vec3.of(radius);
        const boundingBox = BoundingBox.corners(staticCenter.minus(radiusVec), staticCenter.plus(radiusVec));

        return new Sphere(center, radius, material, boundingBox);
    }

    public static moving(startCenter: Point3, endCenter: Point3, radius: number, material: Material) {
        const center = new Ray(startCenter, endCenter.minus(startCenter), 0);

        const radiusVec = Vec3.of(radius);
        const startBox = BoundingBox.corners(center.at(0).minus(radiusVec), center.at(0).plus(radiusVec));
        const endBox = BoundingBox.corners(center.at(1).minus(radiusVec), center.at(1).plus(radiusVec));
        const boundingBox = startBox.join(endBox);

        return new Sphere(center, radius, material, boundingBox);
    }

    constructor(
        private center: Ray,
        private radius: number,
        private material: Material,
        public boundingBox: BoundingBox,
    ) {
        super();
    }

    public override hit(ray: Ray, rayT: Interval): HitResult | null {
        // Heavily optimized code version of using the quadratic formula to solve sphere equation x^2+y^2+z^2=r^2 using vectors
        const currentCenter = this.center.at(ray.time);
        const oc = currentCenter.minus(ray.origin);
        const a = ray.direction.lengthSquared;
        const h = ray.direction.dot(oc);
        const c = oc.lengthSquared - this.radius ** 2;
        const discriminant = h * h - a * c;
        if (discriminant < 0) return null;

        const sqrtD = Math.sqrt(discriminant);

        const root1 = (h - sqrtD) / a;
        const root2 = (h + sqrtD) / a;
        const root = rayT.surrounds(root1) ? root1 : rayT.surrounds(root2) ? root2 : null;
        if (!root) return null;

        const t = root;
        const point = ray.at(t);
        const outwardNormal = point.minus(currentCenter).div(this.radius);

        const theta = Math.acos(-point.y);
        const phi = Math.atan2(-point.z, point.x) + Math.PI;
        const u = phi / (2 * Math.PI);
        const v = theta / Math.PI;

        return new HitResult({
            material: this.material,
            t,
            point,
            ray,
            outwardNormal,
            u,
            v,
        });
    }
}

export class HittableList extends Hittable {
    private objects: Hittable[] = [];

    constructor(objects: Hittable[] = []) {
        super();
        for (const object of objects) {
            this.add(object);
        }
    }

    public hit(ray: Ray, rayT: Interval): HitResult | null {
        let bestHit = null;
        let closestSoFar = rayT.max;

        for (const object of this.objects) {
            const hit = object.hit(ray, new Interval(rayT.min, closestSoFar));
            if (hit) {
                bestHit = hit;
                closestSoFar = hit.t;
            }
        }

        return bestHit;
    }

    public add(object: Hittable): HittableList {
        this.objects.push(object);
        this.boundingBox = this.boundingBox.join(object.boundingBox);
        return this;
    }

    public toBVH() {
        return new BoundingVolumeHierarchy(this.objects);
    }
}

export class BoundingVolumeHierarchy extends Hittable {
    private left: Hittable;
    private right: Hittable;

    constructor(objects: Hittable[]) {
        super();

        for (const object of objects) {
            this.boundingBox = this.boundingBox.join(object.boundingBox);
        }

        const axis = this.boundingBox.longestAxis();

        if (objects.length === 1) {
            this.left = this.right = objects[0];
        } else if (objects.length == 2) {
            this.left = objects[0];
            this.right = objects[1];
        } else {
            objects.sort((a, b) => a.boundingBox.index(axis).min - b.boundingBox.index(axis).min);
            const mid = Math.floor(objects.length / 2);
            this.left = new BoundingVolumeHierarchy(objects.slice(0, mid));
            this.right = new BoundingVolumeHierarchy(objects.slice(mid));
        }
    }

    public hit(ray: Ray, rayT: Interval): HitResult | null {
        if (!this.boundingBox.hit(ray, rayT)) {
            return null;
        }

        const hitLeft = this.left.hit(ray, rayT);
        const hitRight = this.right.hit(ray, new Interval(rayT.min, hitLeft ? hitLeft.t : rayT.max));

        return hitRight || hitLeft;
    }
}

export class Quad extends Hittable {
    private normal: Vec3;
    private D: number;
    private w: Vec3;

    constructor(
        private Q: Point3,
        private u: Vec3,
        private v: Vec3,
        private material: Material,
    ) {
        super();
        const n = u.cross(v);
        this.normal = n.unitVector;
        this.D = this.normal.dot(Q);
        this.w = n.div(n.dot(n));

        this.boundingBox = BoundingBox.corners(Q, Q.plus(u).plus(v)).join(BoundingBox.corners(Q.plus(u), Q.plus(v)));
    }

    public hit(ray: Ray, rayT: Interval): HitResult | null {
        const denominator = this.normal.dot(ray.direction);

        if (Math.abs(denominator) < 1e-8) {
            return null;
        }

        const t = (this.D - this.normal.dot(ray.origin)) / denominator;
        if (!rayT.contains(t)) {
            return null;
        }

        const point = ray.at(t);
        const planarHitPoint = point.minus(this.Q);
        const u = this.w.dot(planarHitPoint.cross(this.v));
        const v = this.w.dot(this.u.cross(planarHitPoint));

        if (!Interval.unit.contains(u) || !Interval.unit.contains(v)) {
            return null;
        }

        return new HitResult({
            material: this.material,
            t,
            point,
            ray,
            outwardNormal: this.normal,
            u,
            v,
        });
    }

    public static box(a: Point3, b: Point3, material: Material) {
        const min = a.min(b);
        const max = a.max(b);

        const dx = new Vec3(max.x - min.x, 0, 0);
        const dy = new Vec3(0, max.y - min.y, 0);
        const dz = new Vec3(0, 0, max.z - min.z);

        return new HittableList([
            new Quad(new Vec3(min.x, min.y, max.z), dx, dy, material),
            new Quad(new Vec3(max.x, min.y, max.z), dz.neg, dy, material),
            new Quad(new Vec3(max.x, min.y, min.z), dx.neg, dy, material),
            new Quad(new Vec3(min.x, min.y, min.z), dz, dy, material),
            new Quad(new Vec3(min.x, max.y, max.z), dx, dz.neg, material),
            new Quad(new Vec3(min.x, min.y, min.z), dx, dz, material),
        ]).toBVH();
    }
}
