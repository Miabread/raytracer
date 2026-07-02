pub mod surface;
pub mod vec3;

use wasm_bindgen::{Clamped, prelude::*};
use web_sys::{ImageData, OffscreenCanvas};

use crate::{
    surface::Ray,
    vec3::{Color, Point, arrow, color, point, vec3},
};

macro_rules! console_log {
    ($($t:tt)*) => (::web_sys::console::log_1(&format!($($t)*).into()))
}

fn hit_sphere(center: Point, radius: f64, ray: Ray) -> f64 {
    let oc = center - ray.origin;
    let a = ray.direction.dot(ray.direction);
    let b = -2.0 * ray.direction.dot(oc);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / 2.0 * a
    }
}

fn ray_color(ray: Ray) -> Color {
    let t = hit_sphere(point(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let normal = (ray.at(t) - vec3(0.0, 0.0, -1.0)).unit_vector();
        return 0.5 * color(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    }

    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    color(1.0, 1.0, 1.0) * (1.0 - a) + color(0.5, 0.7, 1.0) * a
}

#[wasm_bindgen]
pub fn draw(canvas: OffscreenCanvas, aspect_ratio: f64) {
    console_error_panic_hook::set_once();

    let performance = js_sys::global()
        .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
        .unwrap()
        .performance()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::OffscreenCanvasRenderingContext2d>()
        .unwrap();

    let image_width = 400usize;
    let image_height = (image_width as f64 / aspect_ratio).max(1.0) as usize;

    canvas.set_width(image_width as _);
    canvas.set_height(image_height as _);

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = point(0.0, 0.0, 0.0);

    let viewport_u = arrow(viewport_width, 0.0, 0.0);
    let viewport_v = arrow(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let first_pixel_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut pixels = vec![0u8; image_width * image_height * 4];
    let mut time_total = 0.0;

    console_log!("start rendering");

    for j in 0..image_height {
        let scanline_start = performance.now();

        for i in 0..image_width {
            let pixel_center =
                first_pixel_location + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let direction = (pixel_center - camera_center).arrow();
            let ray = Ray::new(camera_center, direction);
            let color = ray_color(ray);

            let index = (j * image_width + i) * 4;
            pixels[index] = (255.0 * color.r()).round() as _;
            pixels[index + 1] = (255.0 * color.g()).round() as _;
            pixels[index + 2] = (255.0 * color.b()).round() as _;
            pixels[index + 3] = 255;
        }

        let scanline_time = scanline_start - performance.now();
        time_total += scanline_time;
        console_log!(
            "scanlines remaining: {}, took {}ms",
            image_height - j,
            scanline_time
        );
    }

    console_log!(
        "finished rendering, took {}ms with average {}ms per scanline",
        time_total,
        time_total / image_height as f64
    );

    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&pixels),
        image_width as _,
        image_height as _,
    )
    .unwrap();
    context.put_image_data(&image_data, 0.0, 0.0).unwrap();
}
