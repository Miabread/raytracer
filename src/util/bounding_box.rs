use std::{mem::swap, ops::Index};

use crate::{
    components::surface::Ray,
    util::{
        interval::{Interval, interval},
        vec3::Point,
    },
};

#[derive(Debug, Clone, Copy, Default)]
pub struct BoundingBox {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl BoundingBox {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn corners(a: Point, b: Point) -> Self {
        Self {
            x: if a.x() <= b.x() {
                interval(a.x(), b.x())
            } else {
                interval(b.x(), a.x())
            },
            y: if a.y() <= b.y() {
                interval(a.y(), b.y())
            } else {
                interval(b.y(), a.y())
            },
            z: if a.z() <= b.z() {
                interval(a.z(), b.z())
            } else {
                interval(b.z(), a.z())
            },
        }
    }

    pub fn join(&self, rhs: BoundingBox) -> Self {
        Self::new(self.x.join(rhs.x), self.y.join(rhs.y), self.z.join(rhs.z))
    }

    pub fn hit(&self, ray: Ray, mut ray_t: Interval) -> bool {
        for axis in 0..3 {
            let inverse = 1.0 / ray.direction[axis];
            let mut t0 = (self[axis].min - ray.origin[axis]) * inverse;
            let mut t1 = (self[axis].max - ray.origin[axis]) * inverse;

            if t0 > t1 {
                swap(&mut t0, &mut t1);
            }

            ray_t.min = ray_t.min.max(t0);
            ray_t.max = ray_t.max.min(t1);

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }
}

impl Index<usize> for BoundingBox {
    type Output = Interval;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("invalid bounding box index"),
        }
    }
}
