pub mod camera;
pub mod material;
pub mod scene;
pub mod surface;
pub mod util;
pub mod vec3;

use js_sys::{Array, ArrayBuffer, Uint32Array};
use wasm_bindgen::prelude::*;

use crate::{
    camera::{Camera, CameraRenderOptions},
    util::worker_scope,
};

#[wasm_bindgen]
pub fn draw(aspect_ratio: f64) {
    console_error_panic_hook::set_once();

    // Scene
    let scene = scene::moving_spheres();

    let mut camera = Camera::new(
        CameraRenderOptions {
            image_width: 400,
            aspect_ratio,
            samples_per_pixel: 100,
            max_depth: 10,
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
