#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod worker;

use std::sync::mpsc;

use eframe::egui::{self, ColorImage};

use crate::worker::{MainMessage, Worker, WorkerConfig, WorkerMessage};

fn main() -> eframe::Result {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };

    eframe::run_native(
        "raytracer_rust_native",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

pub struct App {
    config: WorkerConfig,
    worker_rx: mpsc::Receiver<WorkerMessage>,
    main_tx: mpsc::Sender<MainMessage>,
    texture: Option<egui::TextureHandle>,
    pixels: Vec<u8>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            WorkerConfig::default()
        };

        let (worker_tx, worker_rx) = mpsc::channel();
        let (main_tx, main_rx) = mpsc::channel();
        Worker::spawn_thread(worker_tx, main_rx, cc.egui_ctx.clone());

        Self {
            config,
            worker_rx,
            main_tx,
            texture: None,
            pixels: vec![],
        }
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.config);
    }

    fn on_exit(&mut self) {
        self.main_tx.send(MainMessage::Close).unwrap();
    }

    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut has_updated = false;

        while let Ok(update) = self.worker_rx.try_recv() {
            has_updated = true;

            match update {
                WorkerMessage::Init {
                    image_width,
                    image_height,
                } => {
                    self.pixels = vec![0u8; image_width * image_height * 3];

                    let data =
                        egui::ColorImage::filled([image_width, image_height], egui::Color32::BLACK);

                    self.texture = Some(ctx.load_texture(
                        "pixel_buffer",
                        data.clone(),
                        egui::TextureOptions::NEAREST,
                    ))
                }

                WorkerMessage::Scanline { pixels } => {
                    let width = self
                        .texture
                        .as_ref()
                        .expect("scanline update before init update")
                        .size()[0];

                    for pixel in pixels {
                        let i = pixel.j * width * 3 + pixel.i * 3;
                        self.pixels[i] = pixel.rgb[0];
                        self.pixels[i + 1] = pixel.rgb[1];
                        self.pixels[i + 2] = pixel.rgb[2];
                    }
                }
            }
        }

        if has_updated {
            let texture = self
                .texture
                .as_mut()
                .expect("scanline update before init update");

            let color_image =
                ColorImage::from_rgb([texture.size()[0], texture.size()[1]], &self.pixels);

            texture.set(color_image, egui::TextureOptions::NEAREST);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Window::new("Controls").show(ui, |ui| {
            if ui.button("Start").clicked() {
                self.main_tx
                    .send(MainMessage::Config(self.config.clone()))
                    .unwrap();
            }

            egui::warn_if_debug_build(ui);
        });

        egui::CentralPanel::default().show(ui, |ui| {
            if let Some(texture) = &self.texture {
                ui.image(texture);
            }
        });
    }
}
