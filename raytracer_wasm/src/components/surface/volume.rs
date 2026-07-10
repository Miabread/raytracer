use crate::{
    components::{
        material::MaterialEnum,
        surface::{Ray, Surface, SurfaceEnum, SurfaceHit},
    },
    util::{
        bounding_box::BoundingBox,
        interval::{Interval, interval},
        vec3::arrow,
    },
};

#[derive(Debug, Clone)]
pub struct ConstantMedium {
    boundary: Box<SurfaceEnum>,
    neg_inverse_density: f64,
    phase_function: MaterialEnum,
}

impl ConstantMedium {
    pub fn new(
        boundary: impl Into<SurfaceEnum>,
        density: f64,
        phase_function: impl Into<MaterialEnum>,
    ) -> Self {
        Self {
            boundary: Box::new(boundary.into()),
            neg_inverse_density: -1.0 / density,
            phase_function: phase_function.into(),
        }
    }
}

impl Surface for ConstantMedium {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<SurfaceHit<'_>> {
        let mut hit1 = self.boundary.hit(ray, Interval::FULL)?;
        let mut hit2 = self
            .boundary
            .hit(ray, interval(hit1.t + 0.0001, f64::INFINITY))?;

        if hit1.t < ray_t.min {
            hit1.t = ray_t.min;
        }
        if hit2.t > ray_t.max {
            hit2.t = ray_t.max;
        }

        if hit1.t >= hit2.t {
            return None;
        }

        if hit1.t < 0.0 {
            hit1.t = 0.0;
        }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
        let hit_distance = self.neg_inverse_density * Interval::UNIT.random_double().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = hit1.t + hit_distance / ray_length;
        let point = ray.at(t);

        Some(SurfaceHit {
            t,
            point,
            material: &self.phase_function,

            // Arbitrary
            normal: arrow(1.0, 0.0, 0.0),
            front_face: true,
            u: 0.0,
            v: 0.0,
        })
    }

    fn bounding_box(&self) -> BoundingBox {
        self.boundary.bounding_box()
    }
}
