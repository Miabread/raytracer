use enum_dispatch::enum_dispatch;

use crate::{
    material::MaterialEnum,
    util::{Interval, interval},
    vec3::{Arrow, Point},
};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Arrow,
}

impl Ray {
    pub fn new(origin: Point, direction: Arrow) -> Self {
        Self { origin, direction }
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
#[derive(Clone)]
pub enum SurfaceEnum {
    Sphere,
    SurfaceList,
}

#[enum_dispatch(SurfaceEnum)]
pub trait Surface {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitResult<'_>>;
}

#[derive(Clone)]
pub struct Sphere {
    center: Point,
    radius: f64,
    material: MaterialEnum,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: impl Into<MaterialEnum>) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            material: material.into(),
        }
    }
}

impl Surface for Sphere {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitResult<'_>> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let mut root = (h - sqrt_d) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrt_d) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - self.center).as_arrow() / self.radius;
        Some(HitResult::new(
            t,
            point,
            ray,
            outward_normal,
            &self.material,
        ))
    }
}

#[derive(Clone, Default)]
pub struct SurfaceList {
    surfaces: Vec<SurfaceEnum>,
}

impl SurfaceList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, surface: impl Into<SurfaceEnum>) {
        self.surfaces.push(surface.into());
    }
}

impl Surface for SurfaceList {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitResult<'_>> {
        let mut best_hit = None;
        let mut closest_so_far = ray_t.max;

        for surface in &self.surfaces {
            if let Some(hit) = surface.hit(ray, interval(ray_t.min, closest_so_far)) {
                closest_so_far = hit.t;
                best_hit = Some(hit);
            }
        }

        best_hit
    }
}
