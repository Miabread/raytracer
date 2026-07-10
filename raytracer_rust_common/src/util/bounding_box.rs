use std::{
    mem::swap,
    ops::{Add, Index},
};

use crate::{
    components::surface::Ray,
    util::{
        interval::{Interval, interval},
        vec3::{Arrow, Point},
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
        let delta = 0.0001;
        Self {
            x: if x.size() > delta { x } else { x.expand(delta) },
            y: if y.size() > delta { y } else { y.expand(delta) },
            z: if z.size() > delta { z } else { z.expand(delta) },
        }
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

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else {
            if self.y.size() > self.z.size() { 1 } else { 2 }
        }
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

impl Add<Arrow> for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: Arrow) -> Self::Output {
        BoundingBox::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}

impl Add<BoundingBox> for Arrow {
    type Output = BoundingBox;

    fn add(self, rhs: BoundingBox) -> Self::Output {
        rhs + self
    }
}
