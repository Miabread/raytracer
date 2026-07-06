use enum_dispatch::enum_dispatch;

use crate::{
    components::material::MaterialEnum,
    util::{
        interval::{Interval, interval},
        vec3::{Arrow, Point, arrow},
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
    center: Ray,
    radius: f64,
    material: MaterialEnum,
}

impl Sphere {
    pub fn stationary(
        static_center: Point,
        radius: f64,
        material: impl Into<MaterialEnum>,
    ) -> Self {
        Self {
            center: Ray::new(static_center, arrow(0.0, 0.0, 0.0), 0.0),
            radius: radius.max(0.0),
            material: material.into(),
        }
    }

    pub fn moving(
        center_start: Point,
        center_end: Point,
        radius: f64,
        material: impl Into<MaterialEnum>,
    ) -> Self {
        Self {
            center: Ray::new(center_start, (center_end - center_start).as_arrow(), 0.0),
            radius: radius.max(0.0),
            material: material.into(),
        }
    }
}

impl Surface for Sphere {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitResult<'_>> {
        let current_center = self.center.at(ray.time);
        let oc = current_center - ray.origin;
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
        let outward_normal = (point - current_center).as_arrow() / self.radius;
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
