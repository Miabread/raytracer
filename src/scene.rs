use std::{f64::consts::PI, rc::Rc};

use crate::{
    camera::CameraSceneOptions,
    material::{Dielectric, Lambert, Material, Metal},
    surface::{Sphere, SurfaceList},
    util::Interval,
    vec3::{Color, color, point},
};

pub struct Scene {
    pub world: SurfaceList,
    pub camera: CameraSceneOptions,
}

pub fn first() -> Scene {
    let mut world = SurfaceList::new();

    let material_ground = Rc::new(Lambert::new(color(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambert::new(color(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.50));
    let material_bubble = Rc::new(Dielectric::new(1.00 / 1.50));
    let material_right = Rc::new(Metal::new(color(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(Sphere::new(
        point(0.0, -100.5, -10.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        point(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));

    world.add(Rc::new(Sphere::new(
        point(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        point(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.add(Rc::new(Sphere::new(
        point(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    Scene {
        world,
        camera: CameraSceneOptions {
            vertical_fov: 20.0,
            look_from: point(-2.0, 2.0, 1.0),
            ..Default::default()
        },
    }
}

pub fn second() -> Scene {
    let mut world = SurfaceList::new();

    let r = (PI / 4.0).cos();

    let material_left = Rc::new(Lambert::new(color(0.0, 0.0, 1.0)));
    let material_right = Rc::new(Lambert::new(color(1.0, 0.0, 0.0)));

    world.add(Rc::new(Sphere::new(point(-r, 0.0, -1.0), r, material_left)));
    world.add(Rc::new(Sphere::new(point(r, 0.0, -1.0), r, material_right)));

    Scene {
        world,
        camera: CameraSceneOptions {
            vertical_fov: 90.0,
            ..Default::default()
        },
    }
}

pub fn moving_spheres() -> Scene {
    let mut world = SurfaceList::new();

    let material_ground = Rc::new(Lambert::new(color(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        point(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    let rand = || Interval::UNIT.random_double();

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rand();
            let center = point(a as f64 + 0.9 * rand(), 0.2, b as f64 + 0.9 * rand());

            if (center - point(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }

            let material = if choose_material < 0.8 {
                let albedo = Color::random(Interval::UNIT) * Color::random(Interval::UNIT);
                Rc::new(Lambert::new(albedo)) as Rc<dyn Material>
            } else if choose_material < 0.95 {
                let albedo = Color::random(Interval::UNIT) * Color::random(Interval::UNIT);
                let fuzz = Interval::new(0.0, 0.5).random_double();
                Rc::new(Metal::new(albedo, fuzz)) as Rc<dyn Material>
            } else {
                Rc::new(Dielectric::new(1.5)) as Rc<dyn Material>
            };

            world.add(Rc::new(Sphere::new(center, 0.2, material)));
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(point(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Rc::new(Lambert::new(color(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(point(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Rc::new(Metal::new(color(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(point(4.0, 1.0, 0.0), 1.0, material3)));

    Scene {
        world,
        camera: CameraSceneOptions {
            vertical_fov: 20.0,
            look_from: point(13.0, 2.0, 3.0),
            look_at: point(0.0, 0.0, 0.0),
            defocus_angle: 0.6,
            focus_distance: 10.0,
            ..Default::default()
        },
    }
}
