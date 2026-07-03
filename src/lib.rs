pub mod camera;
pub mod surface;
pub mod util;
pub mod vec3;

use std::rc::Rc;

use wasm_bindgen::{Clamped, prelude::*};
use web_sys::{ImageData, OffscreenCanvas};

use crate::{
    camera::{Camera, CameraRenderOptions, CameraSceneOptions},
    surface::{Sphere, SurfaceList},
    vec3::point,
};

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

    // Image

    let image_width = 400usize;
    let image_height = (image_width as f64 / aspect_ratio).max(1.0) as usize;

    canvas.set_width(image_width as _);
    canvas.set_height(image_height as _);

    // World
    let mut world = SurfaceList::new();

    world.add(Rc::new(Sphere::new(point(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(point(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new(
        CameraRenderOptions {
            image_width,
            aspect_ratio,
            samples_per_pixel: 10,
        },
        CameraSceneOptions::default(),
    );

    let mut pixels = vec![0u8; image_width * image_height * 4];
    camera.render(&world, &performance, &mut pixels);

    // Upload

    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&pixels),
        image_width as _,
        image_height as _,
    )
    .unwrap();
    context.put_image_data(&image_data, 0.0, 0.0).unwrap();
}
