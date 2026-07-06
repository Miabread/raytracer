use crate::{
    components::{
        material::MaterialEnum,
        surface::{HitResult, Ray, Surface},
    },
    util::{
        bounding_box::BoundingBox,
        interval::Interval,
        vec3::{Point, arrow, point},
    },
};

#[derive(Debug, Clone)]
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
            0.0,
            0.0,
        ))
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}
