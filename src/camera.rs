use js_sys::Math;

use crate::{
    surface::{Ray, Surface},
    util::{Interval, interval},
    vec3::{Arrow, Color, Point, arrow, color, point, vec3},
};

#[derive(Debug, Clone, Copy)]
pub struct CameraRenderOptions {
    pub image_width: usize,
    pub aspect_ratio: f64,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
}

impl Default for CameraRenderOptions {
    fn default() -> Self {
        Self {
            image_width: 400,
            aspect_ratio: 16.0 / 9.0,
            samples_per_pixel: 100,
            max_depth: 10,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CameraSceneOptions {
    // pub vertical_fox: f64,
    // pub look_from: Point,
    // pub look_at: Point,
    // pub v_up: Arrow,
    // pub defocus_angle: f64,
    // pub focus_distance: f64,
}

#[allow(clippy::derivable_impls)]
impl Default for CameraSceneOptions {
    fn default() -> Self {
        Self {}
    }
}

struct CameraComputed {
    image_height: usize,
    center: Point,
    first_pixel_location: Point,
    pixel_delta_u: Arrow,
    pixel_delta_v: Arrow,
}

#[allow(dead_code)]
pub struct Camera {
    render: CameraRenderOptions,
    scene: CameraSceneOptions,
    computed: CameraComputed,
    pixel_color: Vec<Color>,
}

impl Camera {
    pub fn new(render: CameraRenderOptions, scene: CameraSceneOptions) -> Self {
        let image_height = (render.image_width as f64 / render.aspect_ratio).max(1.0) as usize;

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (render.image_width as f64 / image_height as f64);
        let center = point(0.0, 0.0, 0.0);

        let viewport_u = arrow(viewport_width, 0.0, 0.0);
        let viewport_v = arrow(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / render.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let first_pixel_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let computed = CameraComputed {
            image_height,
            center,
            first_pixel_location,
            pixel_delta_u,
            pixel_delta_v,
        };

        let pixel_color = vec![color(0.0, 0.0, 0.0); render.image_width * image_height];

        Camera {
            render,
            scene,
            computed,
            pixel_color,
        }
    }

    pub fn render_scanline(
        &mut self,
        world: &impl Surface,
        write_pixel: &mut impl FnMut([usize; 2], [u8; 3]),
    ) {
        // By protocol, first pixel sent determines canvas width and height, so we make sure to start the loop with it
        for n in 0..self.render.samples_per_pixel {
            for j in (0..self.computed.image_height).rev() {
                for i in (0..self.render.image_width).rev() {
                    self.render_pixel(i, j, n, world, write_pixel);
                }
            }
        }
    }

    pub fn render_shotgun(
        &mut self,
        world: &impl Surface,
        write_pixel: &mut impl FnMut([usize; 2], [u8; 3]),
    ) {
        // By protocol, first pixel sent determines canvas width and height, so we make sure to manually render it
        self.render_pixel(
            self.render.image_width - 1,
            self.computed.image_height - 1,
            0,
            world,
            write_pixel,
        );

        let mut pixel_count = vec![0; self.render.image_width * self.computed.image_height];

        loop {
            let i = Math::floor(Math::random() * self.render.image_width as f64) as usize;
            let j = Math::floor(Math::random() * self.computed.image_height as f64) as usize;

            let index = j * self.render.image_width + i;
            pixel_count[index] += 1;
            let n = pixel_count[index];

            self.render_pixel(i, j, n, world, write_pixel);
        }
    }

    pub fn render_pixel(
        &mut self,
        i: usize,
        j: usize,
        n: usize,
        world: &impl Surface,
        write_pixel: &mut impl FnMut([usize; 2], [u8; 3]),
    ) {
        let index = j * self.render.image_width + i;

        let pixel_color = self.pixel_color[index];

        let ray = self.get_ray(i as f64, j as f64);
        let color = Self::get_color(ray, self.render.max_depth, world);

        let pixel_color = pixel_color + color;
        self.pixel_color[index] = pixel_color;

        write_pixel([i, j], self.convert_color(pixel_color / n as f64));
    }

    fn get_color(ray: Ray, depth: usize, world: &impl Surface) -> Color {
        if depth == 0 {
            return color(0.0, 0.0, 0.0);
        }

        let Some(hit) = world.hit(ray, interval(0.001, f64::INFINITY)) else {
            let unit_direction = ray.direction.unit_vector();
            let a = 0.5 * (unit_direction.y() + 1.0);
            return color(1.0, 1.0, 1.0) * (1.0 - a) + color(0.5, 0.7, 1.0) * a;
        };

        let Some(mat_hit) = hit.material.clone().scatter(ray, hit) else {
            return color(0.0, 0.0, 0.0);
        };

        mat_hit.attenuation * Self::get_color(mat_hit.scattered, depth - 1, world)
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let offset = arrow(
            Interval::UNIT.random_double() - 0.5,
            Interval::UNIT.random_double() - 0.5,
            0.0,
        );
        let pixel_sample = self.computed.first_pixel_location
            + ((i + offset.x()) * self.computed.pixel_delta_u)
            + ((j + offset.y()) * self.computed.pixel_delta_v);

        let origin = self.computed.center;
        let direction = (pixel_sample - origin).as_arrow();
        Ray::new(origin, direction)
    }

    fn convert_color(&self, color: Color) -> [u8; 3] {
        let intensity = interval(0.000, 0.999);
        let color = color.map(|a| {
            if a > 0.0 {
                256.0 * intensity.clamp(a.sqrt())
            } else {
                0.0
            }
        });

        [color.r() as _, color.g() as _, color.b() as _]
    }
}
