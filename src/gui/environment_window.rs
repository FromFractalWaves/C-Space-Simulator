// src/gui/environment_window.rs
use eframe::egui;
use nalgebra::{Vector3, Matrix3};
use crate::plants::tropisms::Environment;
use std::sync::{Arc, Mutex};

pub struct EnvironmentWindow {
    environment: Arc<Mutex<Environment>>, // Shared reference to the environment
}

impl EnvironmentWindow {
    pub fn new(environment: Arc<Mutex<Environment>>) -> Self {
        Self { environment }
    }
}

impl eframe::App for EnvironmentWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let env = self.environment.lock().unwrap();
        egui::Window::new("Environment Overview")
            .id(egui::Id::new("environment_overview"))
            .default_pos([600.0, 0.0])
            .default_size([300.0, 400.0])
            .show(ctx, |ui| {
                ui.heading("Environment Metrics");

                ui.group(|ui| {
                    ui.label(format!("Light Position: {:?}", env.light_pos));
                    ui.label(format!("Water Position: {:?}", env.water_pos));
                    ui.label(format!("Gravity: {:?}", env.gravity));
                    ui.label(format!("Light Intensity: {:.2}", env.light_intensity));
                    ui.label(format!("Water Level: {:.2}", env.water_level));
                    ui.label(format!("Obstacles: {}", env.obstacles.len()));
                });

                ui.group(|ui| {
                    ui.label("Metric Tensor");
                    let tensor = env.metric_tensor;
                    ui.label(format!("{:>8.4} {:>8.4} {:>8.4}", tensor[(0, 0)], tensor[(0, 1)], tensor[(0, 2)]));
                    ui.label(format!("{:>8.4} {:>8.4} {:>8.4}", tensor[(1, 0)], tensor[(1, 1)], tensor[(1, 2)]));
                    ui.label(format!("{:>8.4} {:>8.4} {:>8.4}", tensor[(2, 0)], tensor[(2, 1)], tensor[(2, 2)]));
                });

                ui.add_space(10.0);
                ui.label(format!("Critical Distortion: {:.2}", env.d_critical));
            });

        ctx.request_repaint();
    }
}