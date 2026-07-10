use std::{
    ops::ControlFlow,
    sync::mpsc::{Receiver, Sender},
    thread,
};

use eframe::egui;
use raytracer_rust_common::{
    camera::{Camera, CameraRenderOptions},
    scene::{BuiltinScene, Scene},
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

                    self.worker_tx
                        .send(WorkerMessage::Init {
                            image_width: camera.image_width(),
                            image_height: camera.image_height(),
                        })
                        .unwrap();

                    self.renderer = Some(Renderer {
                        scanline: 0,
                        camera,
                        scene,
                        iterations: 0,
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
}

impl Renderer {
    fn render_scanline(&mut self) -> Vec<Pixel> {
        let mut pixels = Vec::with_capacity(self.camera.image_width());

        for i in 0..self.camera.image_width() {
            let j = self.scanline;
            let n = self.iterations;
            let rgb = self.camera.render_pixel(i, j, n, &self.scene.world);
            pixels.push(Pixel { i, j, rgb });
        }

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
