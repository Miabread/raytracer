use std::sync::Arc;

use enum_dispatch::enum_dispatch;

use crate::util::{
    interval::{Interval, interval},
    vec3::{Arrow, Point, arrow},
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

    fn turbulence(&self, mut point: Point, depth: usize) -> f64 {
        let mut accumulator = 0.0;
        let mut weight = 1.0;

        for _ in 0..depth {
            accumulator += weight * self.noise(point);
            weight *= 0.5;
            point = 2.0 * point;
        }

        accumulator.abs()
    }

    fn shared(self) -> Shared
    where
        Self: std::marker::Sized,
    {
        Shared {
            inner: Arc::new(self.into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Shared {
    inner: Arc<NoiseEnum>,
}

impl Noise for Shared {
    fn noise(&self, point: Point) -> f64 {
        self.inner.noise(point)
    }
}

#[derive(Debug, Clone)]
pub struct Perlin {
    random: Box<[Arrow; Perlin::POINT_COUNT]>,
    perm_x: Box<[usize; Perlin::POINT_COUNT]>,
    perm_y: Box<[usize; Perlin::POINT_COUNT]>,
    perm_z: Box<[usize; Perlin::POINT_COUNT]>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut this = Self {
            random: Box::new([arrow(0.0, 0.0, 0.0); Self::POINT_COUNT]),
            perm_x: Box::new([0; Self::POINT_COUNT]),
            perm_y: Box::new([0; Self::POINT_COUNT]),
            perm_z: Box::new([0; Self::POINT_COUNT]),
        };

        for n in this.random.iter_mut() {
            *n = Arrow::random(Interval::DIAM).unit_vector();
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
        let mut i = n - 1;
        while i > 0 {
            let target = interval(0.0, i as f64).random_integer();
            p.swap(i, target);
            i -= 1;
        }
    }

    #[allow(clippy::needless_range_loop)]
    fn perlin_interpolation(c: &[[[Arrow; 2]; 2]; 2], point: Point) -> f64 {
        let u = point.x() - point.x().floor();
        let v = point.y() - point.y().floor();
        let w = point.z() - point.z().floor();

        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let mut accumulator = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let meow = c[i][j][k];
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;

                    let weight = arrow(u - i, v - j, w - k);
                    accumulator += (i * u + (1.0 - i) * (1.0 - u))
                        * (j * v + (1.0 - j) * (1.0 - v))
                        * (k * w + (1.0 - k) * (1.0 - w))
                        * meow.dot(weight);
                }
            }
        }
        accumulator
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

impl Noise for Perlin {
    #[allow(clippy::needless_range_loop)]
    fn noise(&self, point: Point) -> f64 {
        let i = point.x().floor() as isize;
        let j = point.y().floor() as isize;
        let k = point.z().floor() as isize;

        let mut c = [[[arrow(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random[self.perm_x[((i + di as isize) & 255) as usize]
                        ^ self.perm_y[((j + dj as isize) & 255) as usize]
                        ^ self.perm_z[((k + dk as isize) & 255) as usize]];
                }
            }
        }

        Self::perlin_interpolation(&c, point)
    }
}
