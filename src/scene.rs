use std::rc::Rc;

use crate::{
    camera::CameraSceneOptions,
    material::{Dielectric, Lambert, Metal},
    surface::{Sphere, Surface, SurfaceList},
    vec3::{color, point},
};

pub struct Scene<T: Surface> {
    pub world: T,
    pub camera: CameraSceneOptions,
}

pub fn first() -> Scene<impl Surface> {
    let mut world = SurfaceList::new();

    let material_ground = Rc::new(Lambert::new(color(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambert::new(color(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.50));
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
        point(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    Scene {
        world,
        camera: CameraSceneOptions::default(),
    }
}
