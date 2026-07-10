use std::f64::consts::PI;

use crate::{
    components::{
        material::{Material, MaterialEnum},
        surface::{Ray, Surface, SurfaceHit, structure::SurfaceList},
    },
    util::{
        bounding_box::BoundingBox,
        interval::Interval,
        vec3::{Arrow, Point, arrow, point},
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
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<SurfaceHit<'_>> {
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

        let theta = (-point.y()).acos();
        let phi = (-point.z()).atan2(point.x()) + PI;
        let u = phi / (2.0 * PI);
        let v = theta / PI;

        Some(SurfaceHit::new(
            t,
            point,
            ray,
            outward_normal,
            &self.material,
            u,
            v,
        ))
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}

#[derive(Debug, Clone)]
pub struct Quad {
    center: Point,
    u: Arrow,
    v: Arrow,
    material: MaterialEnum,

    bounding_box: BoundingBox,
    normal: Arrow,
    dot: f64,
    w: Arrow,
}

impl Quad {
    pub fn new(center: Point, u: Arrow, v: Arrow, material: impl Into<MaterialEnum>) -> Self {
        let bounding_box = BoundingBox::corners(center, center + u + v)
            .join(BoundingBox::corners(center + u, center + v));

        let n = u.cross(v);
        let normal = n.unit_vector();
        let dot = normal.dot(center);
        let w = n / n.dot(n);

        Self {
            center,
            u,
            v,
            material: material.into(),

            bounding_box,
            normal,
            dot,
            w,
        }
    }

    pub fn cube(point_a: Point, point_b: Point, material: impl Into<MaterialEnum>) -> SurfaceList {
        let mut sides = SurfaceList::new();
        let material = material.into().shared();

        let min = point_a.min(point_b);
        let max = point_a.max(point_b);

        let dx = arrow(max.x() - min.x(), 0.0, 0.0);
        let dy = arrow(0.0, max.y() - min.y(), 0.0);
        let dz = arrow(0.0, 0.0, max.z() - min.z());

        sides.add(Quad::new(
            point(min.x(), min.y(), max.z()),
            dx,
            dy,
            material.clone(),
        ));
        sides.add(Quad::new(
            point(max.x(), min.y(), max.z()),
            -dz,
            dy,
            material.clone(),
        ));
        sides.add(Quad::new(
            point(max.x(), min.y(), min.z()),
            -dx,
            dy,
            material.clone(),
        ));
        sides.add(Quad::new(
            point(min.x(), min.y(), min.z()),
            dz,
            dy,
            material.clone(),
        ));
        sides.add(Quad::new(
            point(min.x(), max.y(), max.z()),
            dx,
            -dz,
            material.clone(),
        ));
        sides.add(Quad::new(
            point(min.x(), min.y(), min.z()),
            dx,
            dz,
            material,
        ));

        sides
    }
}

impl Surface for Quad {
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<SurfaceHit<'_>> {
        let denominator = self.normal.dot(ray.direction);

        if denominator.abs() < 1e-8 {
            return None;
        }

        let t = (self.dot - self.normal.dot(ray.origin)) / denominator;
        if !ray_t.contains(t) {
            return None;
        }

        let point = ray.at(t);
        let planar_hit = point - self.center;
        let alpha = self.w.dot(planar_hit.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hit));

        if !Interval::UNIT.contains(alpha) || !Interval::UNIT.contains(beta) {
            return None;
        }

        Some(SurfaceHit::new(
            t,
            point,
            ray,
            self.normal,
            &self.material,
            alpha,
            beta,
        ))
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}
