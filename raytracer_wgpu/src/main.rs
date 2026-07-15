#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

pub mod setup;

use eframe::{egui, egui_wgpu};

use crate::setup::{CustomTriangleCallback, TriangleRenderResources};

fn main() -> eframe::Result {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };

    eframe::run_native(
        "raytracer_wgpu",
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

pub struct App {
    angle: f32,
}

impl App {
    pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Self {
        TriangleRenderResources::create(cc);
        Self { angle: 0.0 }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Window::new("Controls").show(ui, |ui| {
            egui::warn_if_debug_build(ui);
        });

        egui::CentralPanel::default().show(ui, |ui| {
            let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::drag());

            self.angle += response.drag_motion().x * 0.01;
            ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                rect,
                CustomTriangleCallback { angle: self.angle },
            ));
        });
    }
}
