import { Ray, type Hittable } from './hittable';
import { degreesToRadians, Interval, Vec3, type Color3, type Point3 } from './util';

export interface CameraRenderOptions {
    imageWidth: number;
    aspectRatio: number;
    samplesPerPixel: number;
    maxDepth: number;
}

export interface CameraSceneOptions {
    verticalFov: number;
    lookFrom: Vec3;
    lookAt: Vec3;
    vUp: Vec3;

    defocusAngle: number;
    focusDistance: number;

    background?: Color3;
}

export class Camera {
    public imageWidth: number;
    public imageHeight: number;

    private center: Point3;
    private upperLeftPixelLocation: Point3;
    private pixelDeltaU: Vec3;
    private pixelDeltaV: Vec3;

    private u: Vec3;
    private v: Vec3;
    private w: Vec3;
    private defocusDiskU: Vec3;
    private defocusDiskV: Vec3;

    constructor(
        private renderOptions: CameraRenderOptions,
        private sceneOptions: CameraSceneOptions,
    ) {
        this.imageWidth = renderOptions.imageWidth;
        this.imageHeight = Math.max(1, Math.trunc(this.imageWidth / renderOptions.aspectRatio));

        this.center = sceneOptions.lookFrom;

        const viewportHeight =
            2.0 * sceneOptions.focusDistance * Math.tan(degreesToRadians(sceneOptions.verticalFov) / 2);
        const viewportWidth = viewportHeight * (this.imageWidth / this.imageHeight);

        this.w = sceneOptions.lookFrom.minus(sceneOptions.lookAt).unitVector;
        this.u = sceneOptions.vUp.cross(this.w).unitVector;
        this.v = this.w.cross(this.u);

        const viewportU = this.u.times(viewportWidth);
        const viewportV = this.v.times(-viewportHeight);

        this.pixelDeltaU = viewportU.div(this.imageWidth);
        this.pixelDeltaV = viewportV.div(this.imageHeight);

        const viewportUpperLeft = this.center
            .minus(this.w.times(sceneOptions.focusDistance))
            .minus(viewportU.div(2))
            .minus(viewportV.div(2));

        this.upperLeftPixelLocation = viewportUpperLeft
            .plus(this.pixelDeltaU.times(0.5))
            .plus(this.pixelDeltaV.times(0.5));

        const defocusRadius = sceneOptions.focusDistance * Math.tan(degreesToRadians(sceneOptions.defocusAngle / 2));
        this.defocusDiskU = this.u.times(defocusRadius);
        this.defocusDiskV = this.v.times(defocusRadius);
    }

    public async render(world: Hittable, data: ImageDataArray) {
        const BATCH_THRESHOLD_MS = 50;
        let lastYieldTime = performance.now();

        const renderTime = performance.now();
        let totalScanLineTime = 0;

        for (let j = 0; j < this.imageHeight; j++) {
            const scanLineStartTime = performance.now();

            for (let i = 0; i < this.imageWidth; i++) {
                let pixelColor = Vec3.zero;
                for (let sample = 0; sample < this.renderOptions.samplesPerPixel; sample++) {
                    const ray = this.getRay(i, j);
                    pixelColor = pixelColor.plus(this.rayColor(ray, this.renderOptions.maxDepth, world));
                }

                const intensity = new Interval(0.0, 0.999);
                const finalPixelColor = pixelColor
                    .times(1 / this.renderOptions.samplesPerPixel)
                    .map((n) => Math.floor(256 * intensity.clamp(Math.sqrt(n))));

                const index = (j * this.imageWidth + i) * 4;
                data[index + 0] = finalPixelColor.r;
                data[index + 1] = finalPixelColor.g;
                data[index + 2] = finalPixelColor.b;
                data[index + 3] = 255;

                const now = performance.now();
                if (now - lastYieldTime > BATCH_THRESHOLD_MS) {
                    await scheduler.yield();
                    lastYieldTime = now;
                }
            }

            const scanLineTime = performance.now() - scanLineStartTime;
            totalScanLineTime += scanLineTime;

            console.log(`Scan lines remaining: ${this.imageHeight - j}, took ${scanLineTime}ms`);
        }

        const averageScanLineTime = totalScanLineTime / this.imageHeight;
        console.log(`Done rendering! Took ${renderTime}ms, average scan line took ${averageScanLineTime}`);
    }

    private getRay(i: number, j: number) {
        const offset = this.sampleSquare();
        const pixelSample = this.upperLeftPixelLocation
            .plus(this.pixelDeltaU.times(i + offset.x))
            .plus(this.pixelDeltaV.times(j + offset.y));

        const origin = this.sceneOptions.defocusAngle <= 0 ? this.center : this.defocusDiskSample();
        const direction = pixelSample.minus(origin);
        const time = Interval.unit.random();

        return new Ray(origin, direction, time);
    }

    private sampleSquare() {
        const interval = new Interval(-0.5, 0.5);
        return new Vec3(interval.random(), interval.random(), 0);
    }

    private defocusDiskSample() {
        const point = Vec3.randomInUnitDisk();
        return this.center.plus(this.defocusDiskU.times(point.x)).plus(this.defocusDiskV.times(point.y));
    }

    private rayColor(ray: Ray, depth: number, world: Hittable): Vec3 {
        if (depth <= 0) {
            return Vec3.zero;
        }

        const hit = world.hit(ray, new Interval(0.001, Infinity));

        if (!hit) {
            if (this.sceneOptions.background) {
                return this.sceneOptions.background;
            }

            const unitDirection = ray.direction.unitVector;
            const a = 0.5 * (unitDirection.y + 1.0);
            return new Vec3(1.0, 1.0, 1.0).times(1.0 - a).plus(new Vec3(0.5, 0.7, 1.0).times(a));
        }

        const emissionColor = hit.material.emitted(hit.u, hit.v, hit.point);
        const materialResult = hit.material.scatter(ray, hit);
        if (!materialResult) {
            return emissionColor;
        }

        const scatterColor = materialResult.attenuation.times(
            this.rayColor(materialResult.scattered, depth - 1, world),
        );

        return emissionColor.plus(scatterColor);
    }
}
