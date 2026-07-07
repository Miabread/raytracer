pub mod primitive;
pub mod structure;

use std::rc::Rc;

use enum_dispatch::enum_dispatch;

use crate::{
    components::{
        material::MaterialEnum,
        surface::{
            primitive::{Quad, Sphere},
            structure::{BoundingVolumeHierarchy, SurfaceList},
        },
    },
    util::{
        bounding_box::BoundingBox,
        interval::Interval,
        vec3::{Arrow, Point},
    },
};

#[enum_dispatch]
#[derive(Debug, Clone)]
pub enum SurfaceEnum {
    Shared,
    Sphere,
    Quad,
    SurfaceList,
    BoundingVolumeHierarchy,
}

#[enum_dispatch(SurfaceEnum)]
pub trait Surface: Into<SurfaceEnum> {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitResult<'_>>;

    fn bounding_box(&self) -> BoundingBox;

    fn shared(self) -> Shared
    where
        Self: std::marker::Sized,
    {
        Shared {
            inner: Rc::new(self.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Shared {
    inner: Rc<SurfaceEnum>,
}

impl Surface for Shared {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitResult<'_>> {
        self.inner.hit(ray, ray_t)
    }

    fn bounding_box(&self) -> BoundingBox {
        self.inner.bounding_box()
    }
}

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
    pub u: f64,
    pub v: f64,
}

impl<'a> HitResult<'a> {
    fn new(
        t: f64,
        point: Point,
        ray: Ray,
        outward_normal: Arrow,
        material: &'a MaterialEnum,
        u: f64,
        v: f64,
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
            u,
            v,
        }
    }
}
