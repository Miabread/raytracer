use std::sync::Arc;

use enum_dispatch::enum_dispatch;

use crate::{
    components::{
        noise::{Noise, NoiseEnum},
        surface::SurfaceHit,
    },
    util::vec3::{Color, color},
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
    fn value(&self, hit: &SurfaceHit<'_>) -> Color;

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
    inner: Arc<TextureEnum>,
}

impl Texture for Shared {
    fn value(&self, hit: &SurfaceHit<'_>) -> Color {
        self.inner.value(hit)
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
    fn value(&self, _hit: &SurfaceHit<'_>) -> Color {
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
    fn value(&self, hit: &SurfaceHit<'_>) -> Color {
        let p = (self.inverse_scale * hit.point).floor();
        let is_even = (p.x() as i32 + p.y() as i32 + p.z() as i32) % 2 == 0;

        if is_even {
            self.even.value(hit)
        } else {
            self.odd.value(hit)
        }
    }
}

#[derive(Debug, Clone)]
pub struct NoiseTexture {
    noise: NoiseEnum,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(noise: impl Into<NoiseEnum>, scale: f64) -> Self {
        Self {
            noise: noise.into(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, hit: &SurfaceHit<'_>) -> Color {
        color(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(self.scale * hit.point))
    }
}
