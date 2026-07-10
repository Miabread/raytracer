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
            samples_per_pixel: 200,
            max_depth: 50,
        },
        scene.camera,
    );

    // Render
    let batch_amount = 100;
    let mut batch = Vec::with_capacity(3 * batch_amount);
    let worker = worker_scope();

    camera.render_scanline(&scene.world, &mut |[i, j], [r, g, b]| {
        let color: u32 = (255 << 24) | ((b as u32) << 16) | ((g as u32) << 8) | r as u32;
        batch.extend_from_slice(&[i as u32, j as u32, color]);

        if batch.len() >= 3 * batch_amount {
            // Surely there's a way to optimize this to avoid the copy?
            let buffer = ArrayBuffer::new(4 * 3 * batch_amount as u32);
            let view = Uint32Array::new(&buffer);
            view.copy_from(&batch);

            worker
                .post_message_with_transfer(&buffer, &Array::of1(&buffer))
                .unwrap();

            batch.clear();
        }
    });
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
