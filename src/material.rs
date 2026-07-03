use crate::{
    surface::{HitResult, Ray},
    vec3::{Arrow, Color, color},
};

#[derive(Debug, Clone, Copy)]
pub struct MaterialResult {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray: Ray, hit: HitResult) -> Option<MaterialResult>;
}

pub struct Lambert {
    pub albedo: Color,
}

impl Lambert {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambert {
    fn scatter(&self, _ray: Ray, hit: HitResult) -> Option<MaterialResult> {
        let mut direction = hit.normal + Arrow::random_unit_vector();

        if direction.near_zero() {
            direction = hit.normal;
        }

        Some(MaterialResult {
            attenuation: self.albedo,
            scattered: Ray::new(hit.point, direction),
        })
    }
}

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
    fn scatter(&self, ray: Ray, hit: HitResult) -> Option<MaterialResult> {
        let reflected = ray.direction.reflect(hit.normal).unit_vector()
            + (self.fuzz * Arrow::random_unit_vector());
        let scattered = Ray::new(hit.point, reflected);

        (scattered.direction.dot(hit.normal) > 0.0).then_some(MaterialResult {
            attenuation: self.albedo,
            scattered,
        })
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, hit: HitResult) -> Option<MaterialResult> {
        let refraction_index = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.unit_vector();
        let cos_theta = -unit_direction.dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if refraction_index * sin_theta > 1.0 {
            unit_direction.reflect(hit.normal)
        } else {
            unit_direction.refract(hit.normal, refraction_index)
        };

        Some(MaterialResult {
            attenuation: color(1.0, 1.0, 1.0),
            scattered: Ray::new(hit.point, direction),
        })
    }
}
