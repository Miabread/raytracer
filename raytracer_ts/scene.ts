import { type CameraSceneOptions } from './camera';
import { BoundingVolumeHierarchy, Hittable, HittableList, Quad, Sphere } from './hittable';
import { Dielectric, DiffuseLight, Lambert, Metal } from './material';
import { Perlin } from './noise';
import { Checker, NoiseTexture, SolidColor, TurbulenceTexture } from './texture';
import { Interval, Vec3 } from './util';

class Scene {
    constructor(
        public world: Hittable,
        public cameraOptions: CameraSceneOptions,
    ) {}
}

export const scene1 = () => {
    const materialGround = new Lambert(new SolidColor(new Vec3(0.8, 0.8, 0.0)));
    const materialCenter = new Lambert(new SolidColor(new Vec3(0.1, 0.2, 0.5)));
    const materialLeft = new Dielectric(1.5);
    const materialBubble = new Dielectric(1.0 / 1.5);
    const materialRight = new Metal(new Vec3(0.8, 0.6, 0.2), 1.0);

    return new Scene(
        new HittableList([
            Sphere.stationary(new Vec3(0.0, -100.5, -1.0), 100.0, materialGround),
            Sphere.stationary(new Vec3(0.0, 0.0, -1.2), 0.5, materialCenter),
            Sphere.stationary(new Vec3(-1.0, 0.0, -1.0), 0.5, materialLeft),
            Sphere.stationary(new Vec3(-1.0, 0.0, -1.0), 0.4, materialBubble),
            Sphere.stationary(new Vec3(1.0, 0.0, -1.0), 0.5, materialRight),
        ]),
        {
            verticalFov: 20,
            lookFrom: new Vec3(-2, 2, 1),
            lookAt: new Vec3(0, 0, -1),
            vUp: new Vec3(0, 1, 0),

            defocusAngle: 10.0,
            focusDistance: 3.4,
        },
    );
};

export const scene2 = () => {
    const R = Math.cos(Math.PI / 4);

    const materialLeft = new Lambert(new SolidColor(new Vec3(0, 0, 1)));
    const materialRight = new Lambert(new SolidColor(new Vec3(1, 0, 0)));

    return new Scene(
        new HittableList([
            Sphere.stationary(new Vec3(-R, 0, -1), R, materialLeft),
            Sphere.stationary(new Vec3(R, 0, -1), R, materialRight),
        ]),
        {
            verticalFov: 90,
            lookFrom: new Vec3(0, 0, 0),
            lookAt: new Vec3(0, 0, -1),
            vUp: new Vec3(0, 1, 0),

            defocusAngle: 0,
            focusDistance: 10,
        },
    );
};

const randomMaterial = () => {
    const chooseMaterial = Interval.unit.random();

    if (chooseMaterial < 0.8) {
        const albedo = Vec3.random(Interval.unit).times(Vec3.random(Interval.unit));
        return new Lambert(new SolidColor(albedo));
    } else if (chooseMaterial < 0.95) {
        const albedo = Vec3.random(new Interval(0, 0.5));
        const fuzz = new Interval(0, 0.5).random();
        return new Metal(albedo, fuzz);
    } else {
        return new Dielectric(1.5);
    }
};

export const movingSpheres = () => {
    const world = new HittableList();

    const groundMaterial = new Lambert(
        new Checker(
            0.32, //
            new SolidColor(new Vec3(0.2, 0.3, 0.1)),
            new SolidColor(new Vec3(0.9, 0.9, 0.9)),
        ),
    );
    world.add(Sphere.stationary(new Vec3(0, -1000, 0), 1000, groundMaterial));

    for (let a = -11; a < 11; a++) {
        for (let b = -11; b < 11; b++) {
            const center = new Vec3(a + 0.9 * Interval.unit.random(), 0.2, b + 0.9 * Interval.unit.random());

            if (center.minus(new Vec3(4, 0.2, 0)).length > 0.9) {
                const endCenter = center.plus(new Vec3(0, new Interval(0, 0.5).random(), 0));
                world.add(Sphere.moving(center, endCenter, 0.2, randomMaterial()));
            }
        }
    }

    const material1 = new Dielectric(1.5);
    world.add(Sphere.stationary(new Vec3(0, 1, 0), 1.0, material1));

    const material2 = new Lambert(new SolidColor(new Vec3(0.4, 0.2, 0.1)));
    world.add(Sphere.stationary(new Vec3(-4, 1, 0), 1.0, material2));

    const material3 = new Metal(new Vec3(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere.stationary(new Vec3(4, 1, 0), 1.0, material3));

    return new Scene(world.toBVH(), {
        verticalFov: 20,
        lookFrom: new Vec3(13, 2, 3),
        lookAt: new Vec3(0, 0, 0),
        vUp: new Vec3(0, 1, 0),

        defocusAngle: 0.6,
        focusDistance: 10,
    });
};

export const checkeredSpheres = () => {
    const world = new HittableList();

    const checker = new Checker(0.32, new SolidColor(new Vec3(0.2, 0.3, 0.1)), new SolidColor(new Vec3(0.9, 0.9, 0.9)));
    world.add(Sphere.stationary(new Vec3(0, -10, 0), 10, new Lambert(checker)));
    world.add(Sphere.stationary(new Vec3(0, 10, 0), 10, new Lambert(checker)));

    return new Scene(world.toBVH(), {
        verticalFov: 20,
        lookFrom: new Vec3(13, 2, 3),
        lookAt: new Vec3(0, 0, 0),
        vUp: new Vec3(0, 1, 0),

        defocusAngle: 0,
        focusDistance: 10,
    });
};

export const perlinSpheres = () => {
    const world = new HittableList();

    const perlin = new TurbulenceTexture(new Perlin(), 4);
    world.add(Sphere.stationary(new Vec3(0, -1000, 0), 1000, new Lambert(perlin)));
    world.add(Sphere.stationary(new Vec3(0, 2, 0), 2, new Lambert(perlin)));

    return new Scene(world.toBVH(), {
        verticalFov: 20,
        lookFrom: new Vec3(13, 2, 3),
        lookAt: new Vec3(0, 0, 0),
        vUp: new Vec3(0, 1, 0),

        defocusAngle: 0,
        focusDistance: 10,
    });
};

export const quads = () => {
    const data = [
        [new Vec3(-3, -2, 5), new Vec3(0, 0, -4), new Vec3(0, 4, 0), new Vec3(1.0, 0.2, 0.2)],
        [new Vec3(-2, -2, 0), new Vec3(4, 0, 0), new Vec3(0, 4, 0), new Vec3(0.2, 1.0, 0.2)],
        [new Vec3(3, -2, 1), new Vec3(0, 0, 4), new Vec3(0, 4, 0), new Vec3(0.2, 0.2, 1.0)],
        [new Vec3(-2, 3, 1), new Vec3(4, 0, 0), new Vec3(0, 0, 4), new Vec3(1.0, 0.5, 0.0)],
        [new Vec3(-2, -3, 5), new Vec3(4, 0, 0), new Vec3(0, 0, -4), new Vec3(0.2, 0.8, 0.8)],
    ];

    const world = new BoundingVolumeHierarchy(
        data.map(([Q, u, v, albedo]) => new Quad(Q, u, v, new Lambert(new SolidColor(albedo)))),
    );

    return new Scene(world, {
        verticalFov: 80,
        lookFrom: new Vec3(0, 0, 9),
        lookAt: new Vec3(0, 0, 0),
        vUp: new Vec3(0, 1, 0),
        defocusAngle: 0,
        focusDistance: 10,
    });
};

export const simpleLight = () => {
    const world = new HittableList();

    const perlin = new NoiseTexture(new Perlin(), 4);
    world.add(Sphere.stationary(new Vec3(0, -1000, 0), 1000, new Lambert(perlin)));
    world.add(Sphere.stationary(new Vec3(0, 2, 0), 2, new Lambert(perlin)));

    const light = new DiffuseLight(new SolidColor(new Vec3(10, 10, 10)));
    world.add(Sphere.stationary(new Vec3(0, 7, 0), 2, light));
    world.add(new Quad(new Vec3(3, 1, -2), new Vec3(2, 0, 0), new Vec3(0, 2, 0), light));

    return new Scene(world.toBVH(), {
        verticalFov: 20,
        lookFrom: new Vec3(26, 3, 6),
        lookAt: new Vec3(0, 2, 0),
        vUp: new Vec3(0, 1, 0),
        defocusAngle: 0,
        focusDistance: 10,
        background: new Vec3(0, 0, 0),
    });
};

export const cornellBox = () => {
    const red = new Lambert(new SolidColor(new Vec3(0.65, 0.05, 0.05)));
    const white = new Lambert(new SolidColor(new Vec3(0.73, 0.73, 0.73)));
    const green = new Lambert(new SolidColor(new Vec3(0.12, 0.45, 0.15)));
    const light = new DiffuseLight(new SolidColor(new Vec3(15, 15, 15)));

    const quads = [
        [new Vec3(555, 0, 0), new Vec3(0, 555, 0), new Vec3(0, 0, 555), green],
        [new Vec3(0, 0, 0), new Vec3(0, 555, 0), new Vec3(0, 0, 555), red],
        [new Vec3(343, 554, 332), new Vec3(-130, 0, 0), new Vec3(0, 0, -105), light],
        [new Vec3(0, 0, 0), new Vec3(555, 0, 0), new Vec3(0, 0, 555), white],
        [new Vec3(555, 555, 555), new Vec3(-555, 0, 0), new Vec3(0, 0, -555), white],
        [new Vec3(0, 0, 555), new Vec3(555, 0, 0), new Vec3(0, 555, 0), white],
    ] as const;

    const world = new HittableList(quads.map(([Q, u, v, material]) => new Quad(Q, u, v, material)));

    world.add(Quad.box(new Vec3(130, 0, 65), new Vec3(295, 165, 230), white));
    world.add(Quad.box(new Vec3(265, 0, 295), new Vec3(430, 330, 460), white));

    return new Scene(world.toBVH(), {
        verticalFov: 40,
        lookFrom: new Vec3(278, 278, -800),
        lookAt: new Vec3(278, 278, 0),
        vUp: new Vec3(0, 1, 0),
        defocusAngle: 0,
        focusDistance: 10,
        background: new Vec3(0, 0, 0),
    });
};
