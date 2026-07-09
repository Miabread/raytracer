use std::rc::Rc;

use enum_dispatch::enum_dispatch;

use crate::{
    components::{
        surface::{Ray, SurfaceHit},
        texture::{Texture, TextureEnum},
    },
    util::{
        interval::Interval,
        vec3::{Arrow, Color, color},
    },
};

#[derive(Debug, Clone, Copy)]
pub struct MaterialHit {
    pub attenuation: Color,
    pub scattered: Ray,
}

#[enum_dispatch]
#[derive(Debug, Clone)]
pub enum MaterialEnum {
    Shared,
    Lambert,
    Metal,
    Dielectric,
    DiffuseLight,
}

#[enum_dispatch(MaterialEnum)]
pub trait Material: Into<MaterialEnum> {
    fn scatter(&self, _ray: Ray, _hit: SurfaceHit) -> Option<MaterialHit> {
        None
    }

    fn emitted(&self, _hit: &SurfaceHit<'_>) -> Color {
        color(0.0, 0.0, 0.0)
    }

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
    inner: Rc<MaterialEnum>,
}

impl Material for Shared {
    fn scatter(&self, ray: Ray, hit: SurfaceHit) -> Option<MaterialHit> {
        self.inner.scatter(ray, hit)
    }

    fn emitted(&self, hit: &SurfaceHit<'_>) -> Color {
        self.inner.emitted(hit)
    }

    fn shared(self) -> Shared
    where
        Self: std::marker::Sized,
    {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Lambert {
    texture: TextureEnum,
}

impl Lambert {
    pub fn new(texture: impl Into<TextureEnum>) -> Self {
        Self {
            texture: texture.into(),
        }
    }
}

impl Material for Lambert {
    fn scatter(&self, ray: Ray, hit: SurfaceHit) -> Option<MaterialHit> {
        let mut direction = hit.normal + Arrow::random_unit_vector();

        if direction.near_zero() {
            direction = hit.normal;
        }

        Some(MaterialHit {
            attenuation: self.texture.value(&hit),
            scattered: Ray::new(hit.point, direction, ray.time),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit: SurfaceHit) -> Option<MaterialHit> {
        let reflected = ray.direction.reflect(hit.normal).unit_vector()
            + (self.fuzz * Arrow::random_unit_vector());
        let scattered = Ray::new(hit.point, reflected, ray.time);

        (scattered.direction.dot(hit.normal) > 0.0).then_some(MaterialHit {
            attenuation: self.albedo,
            scattered,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Dielectric {
    pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, hit: SurfaceHit) -> Option<MaterialHit> {
        let refraction_index = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.unit_vector();
        let cos_theta = -unit_direction.dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_index * sin_theta > 1.0;

        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        let reflectance = r0 + (1.0 - r0) * (1.0 - cos_theta).powf(5.0);

        let direction = if cannot_refract || reflectance > Interval::UNIT.random_double() {
            unit_direction.reflect(hit.normal)
        } else {
            unit_direction.refract(hit.normal, refraction_index)
        };

        Some(MaterialHit {
            attenuation: color(1.0, 1.0, 1.0),
            scattered: Ray::new(hit.point, direction, ray.time),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DiffuseLight {
    texture: TextureEnum,
}

impl DiffuseLight {
    pub fn new(texture: impl Into<TextureEnum>) -> Self {
        Self {
            texture: texture.into(),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, hit: &SurfaceHit<'_>) -> Color {
        self.texture.value(hit)
    }
}
