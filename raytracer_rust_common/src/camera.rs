use crate::{
    components::{
        material::Material,
        surface::{Ray, Surface},
    },
    util::{
        interval::{Interval, interval},
        vec3::{Arrow, Color, Point, arrow, color, point},
    },
};

#[derive(Debug, Clone, Copy)]
pub struct CameraRenderOptions {
    pub image_width: usize,
    pub aspect_ratio: f64,
    pub max_depth: usize,
}

impl Default for CameraRenderOptions {
    fn default() -> Self {
        Self {
            image_width: 400,
            aspect_ratio: 16.0 / 9.0,
            max_depth: 10,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Background {
    Sky,
    Solid(Color),
}

#[derive(Debug, Clone, Copy)]
pub struct CameraSceneOptions {
    pub vertical_fov: f64,
    pub look_from: Point,
    pub look_at: Point,
    pub v_up: Arrow,
    pub defocus_angle: f64,
    pub focus_distance: f64,
    pub background: Background,
}

impl Default for CameraSceneOptions {
    fn default() -> Self {
        Self {
            vertical_fov: 90.0,
            look_from: point(0.0, 0.0, 0.0),
            look_at: point(0.0, 0.0, -1.0),
            v_up: arrow(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_distance: 10.0,
            background: Background::Sky,
        }
    }
}

struct CameraComputed {
    image_height: usize,
    first_pixel_location: Point,
    pixel_delta_u: Arrow,
    pixel_delta_v: Arrow,
    defocus_disk_u: Arrow,
    defocus_disk_v: Arrow,
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

        // Viewport dimensions
        let theta = scene.vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * scene.focus_distance;
        let viewport_width = viewport_height * (render.image_width as f64 / image_height as f64);

        // Camera basis vectors
        let w = (scene.look_from - scene.look_at).unit_vector().as_arrow();
        let u = scene.v_up.cross(w).unit_vector();
        let v = w.cross(u);

        // Pixel basis vectors
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / render.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            scene.look_from - (scene.focus_distance * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let first_pixel_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Defocus disk basis vectors
        let defocus_radius = scene.focus_distance * (scene.defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let computed = CameraComputed {
            image_height,
            first_pixel_location,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u,
            defocus_disk_v,
        };

        let pixel_color = vec![color(0.0, 0.0, 0.0); render.image_width * image_height];

        Camera {
            render,
            scene,
            computed,
            pixel_color,
        }
    }

    pub fn render_pixel(&mut self, i: usize, j: usize, n: usize, world: &impl Surface) -> [u8; 3] {
        let index = j * self.render.image_width + i;

        let pixel_color = self.pixel_color[index];

        let ray = self.get_ray(i as f64, j as f64);
        let color = self.get_color(ray, self.render.max_depth, world);

        let pixel_color = pixel_color + color;
        self.pixel_color[index] = pixel_color;

        self.convert_color(pixel_color / n as f64)
    }

    fn get_color(&self, ray: Ray, depth: usize, world: &impl Surface) -> Color {
        if depth == 0 {
            return color(0.0, 0.0, 0.0);
        }

        let Some(hit) = world.hit(ray, interval(0.001, f64::INFINITY)) else {
            return match self.scene.background {
                Background::Solid(color) => color,
                Background::Sky => {
                    let unit_direction = ray.direction.unit_vector();
                    let a = 0.5 * (unit_direction.y() + 1.0);
                    color(1.0, 1.0, 1.0) * (1.0 - a) + color(0.5, 0.7, 1.0) * a
                }
            };
        };

        let emission_color = hit.material.emitted(&hit);

        let Some(mat_hit) = hit.material.scatter(ray, hit) else {
            return emission_color;
        };

        let scatter_color =
            mat_hit.attenuation * self.get_color(mat_hit.scattered, depth - 1, world);

        emission_color + scatter_color
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

        let origin = if self.scene.defocus_angle <= 0.0 {
            self.scene.look_from
        } else {
            let p = Point::random_in_unit_disk();
            self.scene.look_from
                + (p.x() * self.computed.defocus_disk_u)
                + (p.y() * self.computed.defocus_disk_v)
        };
        let direction = (pixel_sample - origin).as_arrow();
        let time = Interval::UNIT.random_double();

        Ray::new(origin, direction, time)
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

    pub fn image_width(&self) -> usize {
        self.render.image_width
    }

    pub fn image_height(&self) -> usize {
        self.computed.image_height
    }
}
