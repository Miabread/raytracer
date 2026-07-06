use std::rc::Rc;

use enum_dispatch::enum_dispatch;

use crate::util::{
    interval::{Interval, interval},
    vec3::Point,
};

#[enum_dispatch]
#[derive(Debug, Clone)]
pub enum NoiseEnum {
    Shared,
    Perlin,
}

#[enum_dispatch(NoiseEnum)]

pub trait Noise: Into<NoiseEnum> {
    fn noise(&self, point: Point) -> f64;

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
    inner: Rc<NoiseEnum>,
}

impl Noise for Shared {
    fn noise(&self, point: Point) -> f64 {
        self.inner.noise(point)
    }
}

#[derive(Debug, Clone)]
pub struct Perlin {
    random_doubles: Box<[f64; Perlin::POINT_COUNT]>,
    perm_x: Box<[usize; Perlin::POINT_COUNT]>,
    perm_y: Box<[usize; Perlin::POINT_COUNT]>,
    perm_z: Box<[usize; Perlin::POINT_COUNT]>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut this = Self {
            random_doubles: Box::new([0.0; Self::POINT_COUNT]),
            perm_x: Box::new([0; Self::POINT_COUNT]),
            perm_y: Box::new([0; Self::POINT_COUNT]),
            perm_z: Box::new([0; Self::POINT_COUNT]),
        };

        for n in this.random_doubles.iter_mut() {
            *n = Interval::UNIT.random_double();
        }

        Self::perlin_generate_perm(this.perm_x.as_mut_slice());
        Self::perlin_generate_perm(this.perm_y.as_mut_slice());
        Self::perlin_generate_perm(this.perm_z.as_mut_slice());

        this
    }

    fn perlin_generate_perm(p: &mut [usize]) {
        for (i, n) in p.iter_mut().enumerate() {
            *n = i;
        }

        Self::permute(p, Self::POINT_COUNT);
    }

    fn permute(p: &mut [usize], n: usize) {
        for i in (0..n).rev() {
            let target = interval(0.0, i as f64).random_integer();
            p.swap(i, target);
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

impl Noise for Perlin {
    fn noise(&self, point: Point) -> f64 {
        let point = (point * 4.0).floor();

        let i = (point.x() as i32 & 255) as usize;
        let j = (point.y() as i32 & 255) as usize;
        let k = (point.z() as i32 & 255) as usize;

        self.random_doubles[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
}
