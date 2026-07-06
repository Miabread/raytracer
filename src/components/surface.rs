use enum_dispatch::enum_dispatch;

use crate::{
    components::material::MaterialEnum,
    util::{
        bounding_box::BoundingBox,
        interval::{Interval, interval},
        vec3::{Arrow, Point, arrow, point},
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

    fn bounding_box(&self) -> BoundingBox;
}

#[derive(Clone)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    material: MaterialEnum,
    bounding_box: BoundingBox,
}

impl Sphere {
    pub fn stationary(
        static_center: Point,
        radius: f64,
        material: impl Into<MaterialEnum>,
    ) -> Self {
        let radius = radius.max(0.0);
        let radius_vec = point(radius, radius, radius);
        Self {
            center: Ray::new(static_center, arrow(0.0, 0.0, 0.0), 0.0),
            radius,
            material: material.into(),
            bounding_box: BoundingBox::corners(
                static_center - radius_vec,
                static_center + radius_vec,
            ),
        }
    }

    pub fn moving(
        center_start: Point,
        center_end: Point,
        radius: f64,
        material: impl Into<MaterialEnum>,
    ) -> Self {
        let center = Ray::new(center_start, (center_end - center_start).as_arrow(), 0.0);
        let radius = radius.max(0.0);
        let radius_vec = point(radius, radius, radius);
        let box_start =
            BoundingBox::corners(center.at(0.0) - radius_vec, center.at(0.0) + radius_vec);
        let box_end =
            BoundingBox::corners(center.at(1.0) - radius_vec, center.at(1.0) + radius_vec);

        Self {
            center,
            radius,
            material: material.into(),
            bounding_box: box_start.join(box_end),
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

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}

#[derive(Clone, Default)]
pub struct SurfaceList {
    surfaces: Vec<SurfaceEnum>,
    bounding_box: BoundingBox,
}

impl SurfaceList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, surface: impl Into<SurfaceEnum>) {
        let surface = surface.into();
        self.bounding_box = self.bounding_box.join(surface.bounding_box());
        self.surfaces.push(surface);
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

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}
