use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
};

use eframe::egui;
use raytracer_rust_common::{
    camera::{Camera, CameraRenderOptions},
    scene::{BuiltinScene, Scene},
};

#[derive(Debug, Clone)]
pub struct Worker {
    camera: Camera,
    scene: Scene,
    iterations: usize,
    scanline: usize,
}

impl Worker {
    pub fn spawn_thread(
        pixel_tx: Sender<Vec<Pixel>>,
        config_rx: Receiver<WorkerConfig>,
        ctx: egui::Context,
    ) {
        thread::spawn(move || {
            let mut worker = None;
            loop {
                for config in config_rx.try_iter() {
                    let scene = config.scene.to_scene();
                    let camera = Camera::new(config.render, scene.camera);
                    worker = Some(Worker {
                        camera,
                        scene,
                        iterations: 0,
                        scanline: 0,
                    });
                }

                let Some(worker) = &mut worker else {
                    continue;
                };

                let mut pixels = Vec::with_capacity(worker.camera.image_width());

                for i in 0..worker.camera.image_width() {
                    let j = worker.scanline;
                    let n = worker.iterations;
                    let rgb = worker.camera.render_pixel(i, j, n, &worker.scene.world);
                    pixels.push(Pixel { i, j, rgb });
                }

                worker.scanline += 1;
                if worker.scanline >= worker.camera.image_height() {
                    worker.scanline = 0;
                    worker.iterations += 1;
                }

                pixel_tx.send(pixels).unwrap();
                ctx.request_repaint();
            }
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub i: usize,
    pub j: usize,
    pub rgb: [u8; 3],
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct WorkerConfig {
    pub render: CameraRenderOptions,
    pub scene: BuiltinScene,
}
