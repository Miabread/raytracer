use wasm_bindgen::{Clamped, prelude::*};
use web_sys::{ImageData, OffscreenCanvas, console};

#[wasm_bindgen]
pub fn draw(canvas: OffscreenCanvas) {
    console_error_panic_hook::set_once();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::OffscreenCanvasRenderingContext2d>()
        .unwrap();

    console::log_1(&"acquired canvas".into());

    context.set_fill_style_str("#FF0000");
    context.fill_rect(10.0, 10.0, 150.0, 100.0);

    let image_width = 20usize;
    let image_height = 20usize;

    canvas.set_width(image_width as _);
    canvas.set_height(image_height as _);

    let mut pixels = vec![0u8; image_width * image_height * 4];

    console::log_1(&"created image data".into());

    for j in 0..image_height {
        console::log_1(&format!("scanlines remaining: {}", image_height - j).into());

        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0f64;

            let index = (j * image_width + i) * 4;
            pixels[index] = (255.0 * r).round() as _;
            pixels[index + 1] = (255.0 * g).round() as _;
            pixels[index + 2] = (255.0 * b).round() as _;
            pixels[index + 3] = 255;
        }
    }

    console::log_1(&"finished rendering".into());

    let image_data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&pixels),
        image_width as _,
        image_height as _,
    )
    .unwrap();
    context.put_image_data(&image_data, 0.0, 0.0).unwrap();

    console::log_1(&"uploaded image data".into());
}
