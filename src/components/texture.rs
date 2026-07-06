use enum_dispatch::enum_dispatch;

use crate::util::vec3::{Color, Point};

#[enum_dispatch]
#[derive(Debug, Clone)]
pub enum TextureEnum {
    SolidColor,
    Checker,
}

#[enum_dispatch(TextureEnum)]
pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Point) -> Color;
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
    fn value(&self, _u: f64, _v: f64, _p: Point) -> Color {
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
    fn value(&self, u: f64, v: f64, p: Point) -> Color {
        let x = (self.inverse_scale * p.x()).floor() as i32;
        let y = (self.inverse_scale * p.y()).floor() as i32;
        let z = (self.inverse_scale * p.z()).floor() as i32;

        if (x + y + z) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
