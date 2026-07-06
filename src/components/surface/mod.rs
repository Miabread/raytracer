pub mod primitive;
pub mod structure;

use enum_dispatch::enum_dispatch;

use crate::{
    components::{
        material::MaterialEnum,
        surface::{
            primitive::Sphere,
            structure::{BoundingVolumeHierarchy, SurfaceList},
        },
    },
    util::{
        bounding_box::BoundingBox,
        interval::Interval,
        vec3::{Arrow, Point},
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Arrow,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point, direction: Arrow, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(self, t: f64) -> Point {
        self.origin + t * self.direction
    }
}

#[derive(Clone)]
pub struct HitResult<'a> {
    pub t: f64,
    pub point: Point,
    pub normal: Arrow,
    pub front_face: bool,
    pub material: &'a MaterialEnum,
}

impl<'a> HitResult<'a> {
    fn new(
        t: f64,
        point: Point,
        ray: Ray,
        outward_normal: Arrow,
        material: &'a MaterialEnum,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Self {
            t,
            point,
            normal,
            front_face,
            material,
        }
    }
}

#[enum_dispatch]
#[derive(Debug, Clone)]
pub enum SurfaceEnum {
    Sphere,
    SurfaceList,
    BoundingVolumeHierarchy,
}

#[enum_dispatch(SurfaceEnum)]
pub trait Surface {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitResult<'_>>;

    fn bounding_box(&self) -> BoundingBox;
}
