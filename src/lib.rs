pub mod camera;
pub mod material;
pub mod scene;
pub mod surface;
pub mod util;
pub mod vec3;

use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{Clamped, prelude::*};
use web_sys::{ImageData, OffscreenCanvas};

use crate::{
    camera::{Camera, CameraRenderOptions},
    util::request_animation_frame,
};

#[wasm_bindgen]
pub fn draw(canvas: OffscreenCanvas, aspect_ratio: f64) {
    console_error_panic_hook::set_once();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::OffscreenCanvasRenderingContext2d>()
        .unwrap();

    // Scene
    let scene = scene::first();

    let camera = Camera::new(
        CameraRenderOptions {
            image_width: 400,
            aspect_ratio,
            samples_per_pixel: 100,
            max_depth: 10,
        },
        scene.camera,
    );

    let image_width = camera.image_width();
    let image_height = camera.image_height();

    canvas.set_width(image_width as _);
    canvas.set_height(image_height as _);

    // Render
    let mut pixels = vec![0u8; image_width * image_height * 4];
    camera.render(&scene.world, |[i, j], [r, g, b]| {
        let index = (j * image_width + i) * 4;
        pixels[index] = r;
        pixels[index + 1] = g;
        pixels[index + 2] = b;
        pixels[index + 3] = 255;
    });

    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&pixels),
        image_width as _,
        image_height as _,
    )
    .unwrap();

    // Keep rendering the static image in case the browser wipes our canvas
    let closure_inner = Rc::new(RefCell::new(None));
    let closure_outer = closure_inner.clone();

    *closure_outer.borrow_mut() = Some(Closure::new(move || {
        context.put_image_data(&image_data, 0.0, 0.0).unwrap();
        request_animation_frame(closure_inner.borrow().as_ref().unwrap());
    }));

    request_animation_frame(closure_outer.borrow().as_ref().unwrap());
}
