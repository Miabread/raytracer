use std::{f64::consts::PI, rc::Rc};

use crate::{
    camera::CameraSceneOptions,
    material::{Dielectric, Lambert, Metal},
    surface::{Sphere, SurfaceList},
    vec3::{color, point},
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
