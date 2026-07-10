use crate::{
    components::surface::{Surface, SurfaceEnum},
    util::{
        bounding_box::BoundingBox,
        interval::Interval,
        vec3::{Arrow, arrow, point},
    },
};

use super::{Ray, SurfaceHit};

#[derive(Debug, Clone)]
pub struct Translate {
    surface: Box<SurfaceEnum>,
    offset: Arrow,
    bounding_box: BoundingBox,
}

impl Translate {
    pub fn new(surface: impl Into<SurfaceEnum>, offset: Arrow) -> Self {
        let surface = Box::new(surface.into());
        let bounding_box = surface.bounding_box() + offset;
        Self {
            surface,
            offset,
            bounding_box,
        }
    }
}

impl Surface for Translate {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<SurfaceHit<'_>> {
        let offset_ray = Ray::new(ray.origin - self.offset, ray.direction, ray.time);

        let mut hit = self.surface.hit(offset_ray, ray_t)?;

        hit.point = hit.point + self.offset;

        Some(hit)
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}

#[derive(Debug, Clone)]
pub struct RotateY {
    surface: Box<SurfaceEnum>,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: BoundingBox,
}

impl RotateY {
    pub fn new(surface: impl Into<SurfaceEnum>, angle: f64) -> Self {
        let surface = Box::new(surface.into());
        let bounding_box = surface.bounding_box();

        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let mut min = point(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = point(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;

                    let x = i * bounding_box.x.max + (1.0 - i) * bounding_box.x.min;
                    let y = j * bounding_box.x.max + (1.0 - j) * bounding_box.x.min;
                    let z = k * bounding_box.x.max + (1.0 - k) * bounding_box.x.min;

                    let x = cos_theta * x + sin_theta * z;
                    let z = -sin_theta * x - cos_theta * z;

                    let tester = arrow(x, y, z);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let bounding_box = BoundingBox::corners(min, max);

        Self {
            surface,
            sin_theta,
            cos_theta,
            bounding_box,
        }
    }
}

impl Surface for RotateY {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<SurfaceHit<'_>> {
        let origin = point(
            (self.cos_theta * ray.origin.x()) - (self.sin_theta * ray.origin.z()),
            ray.origin.y(),
            (self.sin_theta * ray.origin.x()) + (self.cos_theta * ray.origin.z()),
        );

        let direction = arrow(
            (self.cos_theta * ray.direction.x()) - (self.sin_theta * ray.direction.z()),
            ray.direction.y(),
            (self.sin_theta * ray.direction.x()) + (self.cos_theta * ray.direction.z()),
        );

        let rotated_ray = Ray::new(origin, direction, ray.time);

        let mut hit = self.surface.hit(rotated_ray, ray_t)?;

        hit.point = point(
            (self.cos_theta * hit.point.x()) + (self.sin_theta * hit.point.z()),
            hit.point.y(),
            (-self.sin_theta * hit.point.x()) + (self.cos_theta * hit.point.z()),
        );

        hit.normal = arrow(
            (self.cos_theta * hit.normal.x()) + (self.sin_theta * hit.normal.z()),
            hit.normal.y(),
            (-self.sin_theta * hit.normal.x()) + (self.cos_theta * hit.normal.z()),
        );

        Some(hit)
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}
