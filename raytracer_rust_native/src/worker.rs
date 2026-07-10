use std::{
    ops::ControlFlow,
    sync::mpsc::{Receiver, Sender},
    thread,
};

use eframe::egui;
use rayon::prelude::*;
use raytracer_rust_common::{
    camera::{Camera, CameraRenderOptions},
    scene::{BuiltinScene, Scene},
    util::vec3::{Color, color},
};

#[derive(Debug)]
pub struct Worker {
    renderer: Option<Renderer>,
    worker_tx: Sender<WorkerMessage>,
    main_rx: Receiver<MainMessage>,
    ctx: egui::Context,
}

impl Worker {
    pub fn spawn_thread(
        worker_tx: Sender<WorkerMessage>,
        main_rx: Receiver<MainMessage>,
        ctx: egui::Context,
    ) {
        thread::Builder::new()
            .name("worker".to_owned())
            .spawn(move || {
                let mut worker = Worker {
                    renderer: None,
                    worker_tx,
                    main_rx,
                    ctx,
                };

                while worker.process().is_continue() {}
            })
            .unwrap();
    }

    fn process(&mut self) -> ControlFlow<()> {
        for message in self.main_rx.try_iter() {
            match message {
                MainMessage::Close => return ControlFlow::Break(()),
                MainMessage::Config(config) => {
                    let scene = config.scene.to_scene();
                    let camera = Camera::new(config.render, scene.camera);

                    let image_width = camera.image_width();
                    let image_height = camera.image_height();
                    let pixel_sums = vec![vec![color(0.0, 0.0, 0.0); image_width]; image_height];

                    self.worker_tx
                        .send(WorkerMessage::Init {
                            image_width,
                            image_height,
                        })
                        .unwrap();

                    self.renderer = Some(Renderer {
                        scanline: 0,
                        camera,
                        scene,
                        iterations: 0,
                        pixel_sums,
                    });
                }
            }
        }

        if let Some(renderer) = &mut self.renderer {
            let pixels = renderer.render_scanline();

            self.worker_tx
                .send(WorkerMessage::Scanline { pixels })
                .unwrap();

            self.ctx.request_repaint();
        }

        ControlFlow::Continue(())
    }
}

#[derive(Debug, Clone)]
pub struct Renderer {
    camera: Camera,
    scene: Scene,
    iterations: usize,
    scanline: usize,
    pixel_sums: Vec<Vec<Color>>,
}

impl Renderer {
    fn render_scanline(&mut self) -> Vec<Pixel> {
        let (pixels, pixel_sums) = self.pixel_sums[self.scanline]
            .par_iter()
            .enumerate()
            .map(|(i, pixel_sum)| {
                let n = self.iterations as f64;
                let j = self.scanline;

                let pixel = self.camera.render_pixel(i, j, &self.scene.world);

                let pixel_sum = *pixel_sum + pixel;
                let rgb = (pixel_sum / n).to_rgb();

                (Pixel { i, j, rgb }, pixel_sum)
            })
            .unzip();

        self.pixel_sums[self.scanline] = pixel_sums;

        self.scanline += 1;
        if self.scanline >= self.camera.image_height() {
            self.scanline = 0;
            self.iterations += 1;
        }

        pixels
    }
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct WorkerConfig {
    pub render: CameraRenderOptions,
    pub scene: BuiltinScene,
}

#[derive(Debug, Clone)]
pub enum MainMessage {
    Config(WorkerConfig),
    Close,
}

#[derive(Debug, Clone)]
pub enum WorkerMessage {
    Init {
        image_width: usize,
        image_height: usize,
    },
    Scanline {
        pixels: Vec<Pixel>,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub i: usize,
    pub j: usize,
    pub rgb: [u8; 3],
}
