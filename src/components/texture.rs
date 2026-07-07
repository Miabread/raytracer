use std::rc::Rc;

use enum_dispatch::enum_dispatch;

use crate::{
    components::noise::{Noise, NoiseEnum},
    util::vec3::{Color, Point, color},
};

#[enum_dispatch]
#[derive(Debug, Clone)]
pub enum TextureEnum {
    Shared,
    SolidColor,
    Checker,
    NoiseTexture,
}

#[enum_dispatch(TextureEnum)]
pub trait Texture: Into<TextureEnum> {
    fn value(&self, u: f64, v: f64, point: Point) -> Color;

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
    inner: Rc<TextureEnum>,
}

impl Texture for Shared {
    fn value(&self, u: f64, v: f64, point: Point) -> Color {
        self.inner.value(u, v, point)
    }
}

#[derive(Debug, Clone)]
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _point: Point) -> Color {
        self.albedo
    }
}

impl From<Color> for TextureEnum {
    fn from(albedo: Color) -> Self {
        SolidColor::new(albedo).into()
    }
}

#[derive(Debug, Clone)]
pub struct Checker {
    inverse_scale: f64,
    even: Box<TextureEnum>,
    odd: Box<TextureEnum>,
}

impl Checker {
    pub fn new(scale: f64, even: impl Into<TextureEnum>, odd: impl Into<TextureEnum>) -> Self {
        Self {
            inverse_scale: 1.0 / scale,
            even: Box::new(even.into()),
            odd: Box::new(odd.into()),
        }
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, point: Point) -> Color {
        let p = (self.inverse_scale * point).floor();
        let is_even = (p.x() as i32 + p.y() as i32 + p.z() as i32) % 2 == 0;

        if is_even {
            self.even.value(u, v, point)
        } else {
            self.odd.value(u, v, point)
        }
    }
}

#[derive(Debug, Clone)]
pub struct NoiseTexture {
    noise: NoiseEnum,
}

impl NoiseTexture {
    pub fn new(noise: impl Into<NoiseEnum>) -> Self {
        Self {
            noise: noise.into(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, point: Point) -> Color {
        color(1.0, 1.0, 1.0) * self.noise.noise(point)
    }
}
