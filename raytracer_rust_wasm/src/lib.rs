use js_sys::{Array, ArrayBuffer, Uint32Array};
use wasm_bindgen::prelude::*;

use raytracer_rust_common::{
    camera::{Camera, CameraRenderOptions},
    scene,
};

#[wasm_bindgen]
pub fn draw(aspect_ratio: f64) {
    console_error_panic_hook::set_once();

    // Scene
    let scene = scene::cornell_smoke();

    let mut camera = Camera::new(
        CameraRenderOptions {
            image_width: 600,
            aspect_ratio,
            max_depth: 50,
        },
        scene.camera,
    );

    // Render
    let samples_per_pixel = 200;
    let mut batch = Vec::with_capacity(camera.image_width());
    let worker = worker_scope();

    // By protocol, first pixel sent determines canvas width and height, so we make sure to start the loop with it
    for n in 1..samples_per_pixel {
        for j in (0..camera.image_height()).rev() {
            for i in (0..camera.image_width()).rev() {
                let [r, g, b] = camera.render_pixel(i, j, n, &scene.world);
                let color: u32 = (255 << 24) | ((b as u32) << 16) | ((g as u32) << 8) | r as u32;
                batch.extend_from_slice(&[i as u32, j as u32, color]);
            }

            // Surely there's a way to optimize this to avoid the copy?
            let buffer = ArrayBuffer::new(4 * 3 * camera.image_width() as u32);
            let view = Uint32Array::new(&buffer);
            view.copy_from(&batch);

            worker
                .post_message_with_transfer(&buffer, &Array::of1(&buffer))
                .unwrap();

            batch.clear();
        }
    }
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (::web_sys::console::log_1(&format!($($t)*).into()))
}

pub fn worker_scope() -> web_sys::DedicatedWorkerGlobalScope {
    js_sys::global()
        .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
        .unwrap()
}
