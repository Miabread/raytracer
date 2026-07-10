#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod worker;

use std::sync::mpsc;

use eframe::egui;

use crate::worker::{Pixel, Worker, WorkerConfig};

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
    pixel_rx: mpsc::Receiver<Vec<Pixel>>,
    config_tx: mpsc::Sender<WorkerConfig>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        let (pixel_tx, pixel_rx) = mpsc::channel();
        let (config_tx, config_rx) = mpsc::channel::<WorkerConfig>();
        Worker::spawn_thread(pixel_tx, config_rx, cc.egui_ctx.clone());

        Self {
            config,
            pixel_rx,
            config_tx,
        }
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.config);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Window::new("Controls").show(ui, |ui| {
            egui::warn_if_debug_build(ui);
        });
    }
}
